use crate::bitvector_vanilla::BitVector;
use crate::bitvector_vanilla::IBitVector;

// The pre-SIMD implementation (used on non-x86_64 architectures)
#[cfg(not(target_arch = "x86_64"))]
use crate::doublehasher::DoubleHasher;
#[cfg(not(target_arch = "x86_64"))]
use crate::doublehasher::IDoubleHasher;

// The SIMD implementation (used on x86_64 architectures)
#[cfg(target_arch = "x86_64")]
use crate::doublehasher_x86_avx::DoubleHasher;
#[cfg(target_arch = "x86_64")]
use crate::doublehasher_x86_avx::IDoubleHasher;

use std::hash::Hash;
use std::marker::PhantomData;

/*
Main optimizations:
    * Implement double-hashing to reduce invocations of hash function from N to 2,
        where N is number of hashes
    * Selected the XxHash64 function due to these posts
        https://aras-p.info/blog/2016/08/02/Hash-Functions-all-the-way-down/
        https://jolynch.github.io/posts/use_fast_data_algorithms/
*/

const MEGABYTE: usize = 1 << 20;

#[derive(Debug, Clone)]
pub struct BloomFilter<T> {
    /// The inner bitvector that keeps track of our hashed values
    bit_masks: usize,
    bitvector: BitVector,
    num_hashes: usize,
    hasher: DoubleHasher<T>,
    phantom: PhantomData<T>,
}

#[allow(dead_code)]
impl<T: Hash> BloomFilter<T> {
    /// Creates a new `BloomFilter` with the specified size and number of hash functions
    pub fn new(num_bits: usize, num_hashes: usize) -> Self {
        // Theoretically, the optimal bitvector size is
        // bitvec_size = num_hashes/ln2 * num_elements
        // where num_elements is expected number of elements.
        let num_elements = MEGABYTE * 4; // expected number of elements
        let optim_size = ((num_hashes * num_elements) as f32 / (2.0_f32).ln()) as usize;
        // If the requested bits num_bits is smaller than the optimal size,
        // we override to optimal size. This doesn't have much of an effect
        // but keeping it anyways
        let mut requested_bits = if num_bits < optim_size {
            optim_size
        } else {
            num_bits
        };

        // make requested_bits 2^N
        let mut bit_shift = 0;
        while (1usize << bit_shift) < requested_bits {
            bit_shift += 1;
        }
        requested_bits = 1usize << bit_shift;
        let bitvector = BitVector::new(requested_bits);
        let size = bitvector.size();

        BloomFilter {
            bit_masks: requested_bits.wrapping_sub(1usize),
            bitvector,
            num_hashes,
            hasher: DoubleHasher::new(size),
            phantom: PhantomData,
        }
    }

    /// Inserts an element into the bloom filter
    pub fn insert(&mut self, elem: &T) {
        self.insert_batch8(elem);
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.contains_batch8(elem)
    }

    fn insert_batch4(&mut self, elem: &T) {
        let (hash1, hash2) = self.hasher.get_hash_values(elem);

        for i in (0..self.num_hashes).step_by(4) {
            let index = self.hasher.get_hash_batch4(i, hash1, hash2, self.bit_masks);
            #[allow(clippy::needless_range_loop)]
            for j in 0..4 {
                if i.wrapping_add(j) >= self.num_hashes {
                    return;
                }
                self.bitvector.set(index[j] as usize, true);
            }
        }
    }

    fn insert_batch8(&mut self, elem: &T) {
        let (hash1, hash2) = self.hasher.get_hash_values(elem);

        for i in (0..self.num_hashes).step_by(8) {
            let index = self.hasher.get_hash_batch8(
                i,
                hash1 as u32,
                hash2 as u32,
                self.bit_masks as u32,
                false,
            );
            #[allow(clippy::needless_range_loop)]
            for j in 0..8 {
                if i.wrapping_add(j) >= self.num_hashes {
                    return;
                }
                self.bitvector.set(index[j] as usize, true);
            }
        }
    }

    fn insert_seq(&mut self, elem: &T) {
        let (hash1, hash2) = self.hasher.get_hash_values(elem);

        for i in 0..self.num_hashes {
            let index = self.hasher.get_ith_hash(i, hash1, hash2, self.bit_masks);
            self.bitvector.set(index, true);
        }
    }

    fn contains_batch4(&self, elem: &T) -> bool {
        let (hash1, hash2) = self.hasher.get_hash_values(elem);

        for i in (0..self.num_hashes).step_by(4) {
            let index = self.hasher.get_hash_batch4(i, hash1, hash2, self.bit_masks);
            #[allow(clippy::needless_range_loop)]
            for j in 0..4 {
                if i.wrapping_add(j) >= self.num_hashes {
                    return true;
                }
                if !self.bitvector.get(index[j] as usize) {
                    return false;
                }
            }
        }
        true
    }

    fn contains_batch8(&self, elem: &T) -> bool {
        let (hash1, hash2) = self.hasher.get_hash_values(elem);

        for i in (0..self.num_hashes).step_by(8) {
            let index = self.hasher.get_hash_batch8(
                i,
                hash1 as u32,
                hash2 as u32,
                self.bit_masks as u32,
                true,
            );
            #[allow(clippy::needless_range_loop)]
            for j in 0..8 {
                if i.wrapping_add(j) >= self.num_hashes {
                    return true;
                }
                if !self.bitvector.get(index[j] as usize) {
                    return false;
                }
            }
        }
        true
    }

    fn contains_seq(&self, elem: &T) -> bool {
        let (hash1, hash2) = self.hasher.get_hash_values(elem);

        for i in 0..self.num_hashes {
            let index = self.hasher.get_ith_hash(i, hash1, hash2, self.bit_masks);
            if !self.bitvector.get(index) {
                return false;
            }
        }
        true
    }
}
