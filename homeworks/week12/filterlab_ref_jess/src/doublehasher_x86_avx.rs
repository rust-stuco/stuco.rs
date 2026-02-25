// This is the SIMD-optimized version of doublehasher.rs.
// We use AVX instead of AVX-512, since our grader infra was on stable Rust,
// and the latter would have required Rust's nightly build.
// This module is only compiled on x86_64 architectures.

use rand::Rng;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
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
#[allow(dead_code)]
pub struct DoubleHasher<T> {
    has_avx2: bool,
    prime_modulus: usize, // A large prime close to 2^31 - 1

    // We can convert "a % b" into "(a * (2^N / b)) >> N" when a < 2^N
    // We need this conversion for SIMD since AVX2 doesn't support remainder and integer division.

    // 2^64 / prime_modulus.
    prime_modulus_recip64: u64,

    // 2^32 / prime_modulus
    prime_modulus_recip32: u32,

    // 2^32 % prime_modulus
    prime_modulus_remainder32: u32,

    hasher1: XxHash64,
    coprime_multiplier: u64,
    phantom: PhantomData<T>,
}

#[allow(dead_code)]
// calculate 2^64/x
fn get_reciprocal64(x: u64) -> u64 {
    let mut q = (11u64 << 63).wrapping_div(x).wrapping_shl(1);
    let r = (11u64 << 63).wrapping_rem(x).wrapping_shl(1);
    if r >= x {
        q = q.wrapping_add(1);
    }
    q
}

#[allow(dead_code)]
impl<T> DoubleHasher<T> {
    pub fn new(size: usize) -> Self {
        const BIT_SHIFT: usize = 31;
        const LARGE_PRIME: usize = (1usize << BIT_SHIFT).wrapping_sub(1);
        const PRIME_MODULUS_RECIP32: u32 = ((1u64 << 32) / (LARGE_PRIME as u64)) as u32;

        // Generate a coprime multiplier specific to this size
        let coprime_multiplier = Self::_generate_optimal_multiplier(size as u64);
        let mut has_avx2 = false;
        #[cfg(target_arch = "x86_64")]
        if is_x86_feature_detected!("avx2") {
            has_avx2 = true;
        }

        DoubleHasher {
            has_avx2,
            prime_modulus: LARGE_PRIME,
            prime_modulus_recip32: PRIME_MODULUS_RECIP32,
            prime_modulus_remainder32: ((1usize << 32) % LARGE_PRIME) as u32,
            prime_modulus_recip64: get_reciprocal64(LARGE_PRIME as u64),
            hasher1: XxHash64::with_seed(0),
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

    fn _get_ith_hash_v5(&self, i: usize, hash1: u64, hash2: u64, bit_masks: usize) -> usize {
        // This is very special to our implementation where we know that size
        //  is a multiple of 8, due to our bitvector being a vector of bytes, 8 bits per byte.
        // As such, we know that any odd number is coprime to size and hardcode 269 directly.
        // If we want DoubleHasher to support sizes that aren't even numbers,
        //  the proper way would be to _generate_coprime(size), which has been implemented
        //  in this file as well.
        // But for the sake of optimization, we use 269 directly.
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

    unsafe fn _get_hash_batch8(&self, i: u32, hash1: u32, hash2: u32, bit_masks: u32) -> __m256i {
        unsafe {
            //let mut tmp:[u32;8] = [0;8];

            let h1 = _mm256_set1_epi32(hash1 as i32);
            let h2 = _mm256_set1_epi32(hash2 as i32);
            let ii = _mm256_add_epi32(
                _mm256_set1_epi32(i as i32),
                _mm256_set_epi32(7, 6, 5, 4, 3, 2, 1, 0),
            );
            let mut result = _mm256_xor_si256(h1, ii);
            result = _mm256_mullo_epi32(result, h2);
            result = _mm256_mullo_epi32(result, _mm256_set1_epi32(self.coprime_multiplier as i32));

            /*
            _mm256_storeu_ps(
                tmp.as_mut_ptr() as *mut f32,
                _mm256_castsi256_ps(result));
            */
            assert!(self.prime_modulus == 0x7fffffff);
            result = avx_mod32_0x7fffffff(result);

            result = _mm256_and_si256(result, _mm256_set1_epi32(bit_masks as i32));
            result
        }
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
        simd_enabled: bool,
    ) -> [u32; 8] {
        let mut result: [u32; 8] = [0; 8];
        if simd_enabled && self.has_avx2 {
            unsafe {
                let r = self._get_hash_batch8(i as u32, hash1, hash2, bit_masks);
                _mm256_storeu_ps(result.as_mut_ptr() as *mut f32, _mm256_castsi256_ps(r));
            }
        } else {
            #[allow(clippy::needless_range_loop)]
            for j in 0..8 {
                result[j] = self._get_ith_hash32(i.wrapping_add(j) as u32, hash1, hash2, bit_masks);
            }
        }
        result
    }

    /// Calculates the i-th hash value using double hashing
    fn get_hash_batch4(&self, i: usize, hash1: u64, hash2: u64, bit_masks: usize) -> [u64; 4] {
        let mut result: [u64; 4] = [0u64; 4];
        #[allow(clippy::needless_range_loop)]
        for j in 0..8 {
            result[j] = self._get_ith_hash_v5(i.wrapping_add(j), hash1, hash2, bit_masks) as u64;
        }
        result
    }
}

// Calulate "a[0..7] / b" (32 bit version)
// 1. a contains 8 32-bit unsigned integers
// 2. recip is "2^32 / b"
// 3. m is "2^32 % b"
#[inline]
#[allow(dead_code)]
unsafe fn avx_div32(a: __m256i, recip: u32, m: u32) -> __m256i {
    unsafe {
        // 1. Calulate a / b = (a * r) >> 32
        //    We need to mimic "_mm256_mulhi_epu32" instruction, which multiplies
        //    two 32-bit integers into 64-bit and then keep the high 32 bit.
        let r = _mm256_set1_epi64x(recip as i64);

        // mask for masking out the high 32-bit portions
        //     [-1, 0, -1, 0, -1, 0, -1, 0]
        let mask = _mm256_set1_epi64x(u32::MAX as i64);

        // ahi: 64-bit unsigned intergers
        //      a[1], a[3], a[5], a[7]
        let ahi = _mm256_and_si256(_mm256_shuffle_epi32(a, 0b10_11_00_01), mask);

        // alo: 4 64-bit unsigned intergers
        //      a[0], a[2], a[4], a[6]
        let alo = _mm256_and_si256(a, mask);

        let rem = _mm256_set1_epi64x(m as i64);

        // xlo: 4 64-bit unsigned integers
        //      a[0] * r + m, a[2] * r + m, a[4] * r + m, a[6] * r + m
        let xlo = _mm256_add_epi64(_mm256_mul_epu32(alo, r), rem);

        // xhi: 4 64-bit unsigned integers
        //      a[1] * r + m, a[3] * r + m, a[5] * r + m, a[7] * r + m
        let xhi = _mm256_add_epi64(_mm256_mul_epu32(ahi, r), rem);

        // y: 8 32-bit unsigned integers extracted from the high 32-bit of the following
        //      a[0] * r, a[2] * r, a[1] * r, a[3] * r
        //      a[4] * r, a[6] * r, a[5] * r, a[7] * r
        let y = _mm256_shuffle_ps(
            _mm256_castsi256_ps(xlo),
            _mm256_castsi256_ps(xhi),
            0b11_01_11_01,
        );
        // quotient of "a[0..7] / b"
        _mm256_shuffle_epi32(_mm256_castps_si256(y), 0b11_01_10_00)
    }
}

// Calulate "a[0..7] / b" (32 bit version)
// 1. a contains 8 32-bit unsigned integers
// 2. recip is "2^32 / b"
// 3. m is "2^32 % b"
#[allow(dead_code)]
unsafe fn avx_mod32(a: __m256i, b: u32, recip: u32, m: u32) -> __m256i {
    unsafe {
        let quotient = avx_div32(a, recip, m);

        let prod = _mm256_mullo_epi32(_mm256_set1_epi32(b as i32), quotient);

        _mm256_sub_epi32(a, prod)
    }
}

#[inline]
unsafe fn avx_cmpge_epu32(a: __m256i, b: __m256i) -> __m256i {
    unsafe { _mm256_cmpeq_epi32(a, _mm256_max_epu32(a, b)) }
}

#[inline]
unsafe fn avx_mod32_0x7fffffff(a: __m256i) -> __m256i {
    unsafe {
        // For 32-bit mod against 2^31-1, we can get the remainder after 2 substractions
        let divisor = _mm256_set1_epi32((1u32 << 31).wrapping_sub(1) as i32);
        let mut mask = avx_cmpge_epu32(a, divisor);
        let first = _mm256_sub_epi32(a, _mm256_and_si256(divisor, mask));
        mask = avx_cmpge_epu32(first, divisor);
        _mm256_sub_epi32(first, _mm256_and_si256(divisor, mask))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_div32() {
        let mut rng = rand::rng();
        let d = DoubleHasher::<u64>::new(1usize << 20);

        // This test verifies that avx_div32 returns the quotients in the correct order.
        let b = d.prime_modulus as u32;
        for i in 0..8 {
            let mut a = [0u32; 8];
            for j in 0..8 {
                a[j] = rng.random_range(0..b);
            }

            // quotient[i] is either 1 or 2 when b is 2^31 - 1
            a[i] = if i < 4 {
                u32::MAX
            } else {
                a[i].wrapping_add(b)
            };

            // Assert: a[i] >= b and a[j] < b for all j <> i.
            let mut q = [0u32; 8];
            let mut rem = [0u32; 8];
            let mut rem2 = [0u32; 8];
            unsafe {
                let tmp = _mm256_loadu_ps(a.as_ptr() as *const f32);
                let qq = avx_div32(
                    _mm256_castps_si256(tmp),
                    d.prime_modulus_recip32,
                    d.prime_modulus_remainder32,
                );
                _mm256_storeu_ps(q.as_mut_ptr() as *mut f32, _mm256_castsi256_ps(qq));
                let rr = avx_mod32(
                    _mm256_castps_si256(tmp),
                    b,
                    d.prime_modulus_recip32,
                    d.prime_modulus_remainder32,
                );
                _mm256_storeu_ps(rem.as_mut_ptr() as *mut f32, _mm256_castsi256_ps(rr));
                let rr2 = avx_mod32_0x7fffffff(_mm256_castps_si256(tmp));
                _mm256_storeu_ps(rem2.as_mut_ptr() as *mut f32, _mm256_castsi256_ps(rr2));
            }

            for j in 0..8 {
                assert_eq!(q[j], (a[j] as u32) / b);
                assert_eq!(rem[j], (a[j] as u32) % b);
                assert_eq!(rem2[j], (a[j] as u32) % b);
            }
        }
    }

    #[test]
    fn test_hash_batch8() {
        let bits = 1usize << 20;
        let d = DoubleHasher::<u64>::new(bits);
        let bit_masks = bits.wrapping_sub(1) as u32;

        for i in (1..=300).filter(|n| n % 3 == 0) {
            let elem = i as u64;
            let (hash1, hash2) = d.get_hash_values(&elem);

            // truncate the hash values
            let h1 = hash1 as u32;
            let h2 = hash2 as u32;

            let a = d.get_hash_batch8(0, h1, h2, bit_masks, true);
            for j in 0..8 {
                let expected = d._get_ith_hash32(j, h1, h2, bit_masks);
                assert_eq!(expected, a[j as usize]);
            }
        }
    }
}
