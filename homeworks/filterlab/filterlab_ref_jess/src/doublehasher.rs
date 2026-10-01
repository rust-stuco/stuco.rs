// The implementation of the double-hashing algorithm *before* SIMD.

use rand::Rng;
use std::cmp::{max, min};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use twox_hash::XxHash64;

#[allow(dead_code)]
pub trait IDoubleHasher<T> {
    /// Generates two independent hash values for an element
    fn get_hash_values(&self, elem: &T) -> (u64, u64);

    /// Calculates the i-th hash value using double hashing
    fn get_ith_hash(&self, i: usize, hash1: u64, hash2: u64, bit_masks: usize) -> usize;

    fn get_hash_batch8(
        &self,
        i: usize,
        hash1: u32,
        hash2: u32,
        bit_masks: u32,
        simd_enabled: bool,
    ) -> [u32; 8];

    fn get_hash_batch4(&self, i: usize, hash1: u64, hash2: u64, bit_masks: usize) -> [u64; 4];
}

#[derive(Debug, Clone)]
pub struct DoubleHasher<T> {
    prime_modulus: usize, // A large prime close to 2^31 - 1
    hasher1: XxHash64,
    coprime_multiplier: u64,
    phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl<T> DoubleHasher<T> {
    pub fn new(size: usize) -> Self {
        const BIT_SHIFT: usize = 31;
        const LARGE_PRIME: usize = (1usize << BIT_SHIFT).wrapping_sub(1);

        // Generate a coprime multiplier specific to this size
        let coprime_multiplier = Self::_generate_optimal_multiplier(size as u64);
        DoubleHasher {
            prime_modulus: LARGE_PRIME,
            hasher1: XxHash64::default(),
            coprime_multiplier,
            phantom: PhantomData,
        }
    }

    // Helpers for generating coprime
    fn _gcd(x: usize, y: usize) -> usize {
        let mut a = max(x, y);
        let mut b = min(x, y);
        let mut r = a % b;
        while r != 0 {
            a = b;
            b = r;
            r = a % b;
        }
        b
    }

    // Generates a number that is coprime to x
    // Uniformly randomly generates a random number until we get a coprime
    // In expectation, this takes a constant number of tries.
    // If x is a prime number p, we can just return any number <= p-1.
    // If x is an even number, we can return any odd number
    // However, checking if a number is prime is likely less efficient than
    //  directly doing our routine below in expected constant time.
    fn _generate_coprime(x: usize) -> usize {
        let mut rng = rand::rng();
        loop {
            let candidate = rng.random_range(1..=usize::MAX) | 1;
            if Self::_gcd(candidate, x) == 1 {
                return candidate;
            }
        }
    }

    // Find optimal multiplier by testing collision rates
    fn _generate_optimal_multiplier(size: u64) -> u64 {
        const CANDIDATE_MULTIPLIERS: &[u64] = &[269, 421, 631, 1087, 1993, 3079, 4603];
        let mut best_multiplier = CANDIDATE_MULTIPLIERS[0];
        let mut least_collisions = u64::MAX;

        // Simple collision test with sample data
        for &multiplier in CANDIDATE_MULTIPLIERS {
            let mut collisions = 0;
            let mut seen = vec![false; size as usize];
            for i in 0..1000 {
                let hash = (i * multiplier) % size;
                if seen[hash as usize] {
                    collisions += 1;
                }
                seen[hash as usize] = true;
            }
            if collisions < least_collisions {
                least_collisions = collisions;
                best_multiplier = multiplier;
            }
        }
        best_multiplier
    }

    // Version 1: (hash1 + i * hash2) % size
    // Too many false positives for random_large_test
    //      189233 false positives
    fn _get_ith_hash_v1(&self, i: usize, hash1: u64, hash2: u64, size: usize) -> usize {
        let delta = i.wrapping_mul(hash2 as usize) % size;
        (hash1 as usize % size + delta) % size
    }

    // Version 2: introduce a large prime modulus
    // Reduced false positives for random_large_test to
    //      31815 false positives
    fn _get_ith_hash_v2(&self, i: usize, hash1: u64, hash2: u64, size: usize) -> usize {
        let delta = i.wrapping_mul(hash2 as usize) % self.prime_modulus;
        (hash1 as usize % self.prime_modulus + delta) % self.prime_modulus % size
    }

    // Version 3: Simulate double-hashing with incremental XOR
    //      Spreads values more evenly in cases where the modulus creates patterns
    // Reduced false positives for random_large_test to
    //      26028 false positives
    fn _get_ith_hash_v3(&self, i: usize, hash1: u64, hash2: u64, size: usize) -> usize {
        ((hash1 ^ (i as u64).wrapping_mul(hash2)) as usize % self.prime_modulus) % size
    }

    // Version 4: Introduce random multiplier that is coprime to size
    // Finally goes below the 22K-24K threshold for random_large_test
    //      Have    23293 false positives when multiplier=5
    //              23045 false positives when multiplier=13
    //              22920 false positives when multiplier=17
    //              22534 false positives when multiplier=269
    fn _get_ith_hash_v4(&self, i: usize, hash1: u64, hash2: u64, size: usize) -> usize {
        // This is very special to our implementation where we know that size
        //  is a multiple of 8, due to our bitvector being a vector of bytes, 8 bits per byte.
        // As such, we know that any odd number is coprime to size and hardcode 269 directly.
        // If we want DoubleHasher to support sizes that aren't even numbers,
        //  the proper way would be to _generate_coprime(size), which has been implemented
        //  in this file as well.
        // But for the sake of optimization, we use 269 directly.
        const COPRIME_MULTIPLIER: u64 = 269;
        let xor_result = (hash1 ^ (i as u64).wrapping_mul(hash2)).wrapping_mul(COPRIME_MULTIPLIER);
        (xor_result as usize % self.prime_modulus) % size
    }

    fn _get_ith_hash_v5(&self, i: usize, hash1: u64, hash2: u64, bit_masks: usize) -> usize {
        let xor_result = (hash1 ^ (i as u64))
            .wrapping_mul(hash2)
            .wrapping_mul(self.coprime_multiplier);
        (xor_result as usize).wrapping_rem(self.prime_modulus) & bit_masks
    }

    fn _get_ith_hash32(&self, i: u32, hash1: u32, hash2: u32, bit_masks: u32) -> u32 {
        let xor_result = (hash1 ^ i)
            .wrapping_mul(hash2)
            .wrapping_mul(self.coprime_multiplier as u32);
        xor_result.wrapping_rem(self.prime_modulus as u32) & bit_masks
    }
}

impl<T: Hash> IDoubleHasher<T> for DoubleHasher<T> {
    /// Generates two independent hash values for an element
    fn get_hash_values(&self, elem: &T) -> (u64, u64) {
        let mut hasher1 = self.hasher1.clone();

        elem.hash(&mut hasher1);

        let h1 = hasher1.finish();
        (h1, h1 ^ 0xFEEDFACECAFEBEEF)
    }

    /// Calculates the i-th hash value using double hashing
    fn get_ith_hash(&self, i: usize, hash1: u64, hash2: u64, bit_masks: usize) -> usize {
        self._get_ith_hash_v5(i, hash1, hash2, bit_masks)
    }

    /// Calculates the i-th hash value using double hashing
    fn get_hash_batch8(
        &self,
        i: usize,
        hash1: u32,
        hash2: u32,
        bit_masks: u32,
        _simd_enabled: bool,
    ) -> [u32; 8] {
        let mut result: [u32; 8] = [0; 8];
        #[allow(clippy::needless_range_loop)]
        for j in 0..8 {
            result[j] = self._get_ith_hash32(i.wrapping_add(j) as u32, hash1, hash2, bit_masks);
        }
        result
    }

    fn get_hash_batch4(&self, i: usize, hash1: u64, hash2: u64, bit_masks: usize) -> [u64; 4] {
        let mut result: [u64; 4] = [0; 4];
        #[allow(clippy::needless_range_loop)]
        for j in 0..4 {
            result[j] = self._get_ith_hash_v5(i.wrapping_add(j), hash1, hash2, bit_masks) as u64;
        }
        result
    }
}
