use crate::bitvector::BitVector;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::marker::PhantomData;

/// An approximate-membership query / probabilistic data structure that supports point lookups.
#[derive(Debug, Clone)]
pub struct BloomFilter<T> {
    /// The inner bitvector / bitset that keeps track of our hashed values.
    bitvector: BitVector,

    /// The number of hashes for each element.
    num_hashes: usize,

    /// A type marker used to express that this `BloomFilter` is generic over a single type.
    ///
    /// Note that this means this `BloomFilter` is _not_ allowed to "store" elements of different
    /// types. Each `BloomFilter` instance is specific to a single type (Monomorphization), but the
    /// type itself is generic.
    ///
    /// We could have gone with the approach of making only hashing generic, but for simplicity we
    /// will only allow the `BloomFilter` to track elements of a single type.
    phantom: PhantomData<T>,
}

impl<T: Hash> BloomFilter<T> {
    /// Creates a new `BloomFilter` given the maximum number of elements that will be inserted into
    /// the filter and a bound on the size of the `BloomFilter`'s bitvector.
    pub fn new(num_bits: usize, num_hashes: usize) -> Self {
        Self {
            bitvector: BitVector::new(num_bits),
            num_hashes,
            phantom: PhantomData,
        }
    }

    /// Inserts an element into the bloom filter.
    ///
    /// Note that this implementation is purposefully slow. We would like you to think of ways to
    /// improve the performance!
    pub fn insert(&mut self, elem: &T) {
        let size = self.bitvector.size();

        let mut hasher = DefaultHasher::new();

        // Provide a starting "seed" for hashing.
        elem.hash(&mut hasher);
        let mut hash = hasher.finish();

        for _ in 0..self.num_hashes {
            // Rehash the hash.
            hash.hash(&mut hasher);
            hash = hasher.finish();

            // Set the bit corresponding to this hash value.
            self.bitvector.set(hash as usize % size, true);
        }
    }

    /// Checks if an element might have been previously inserted into the bloom filter.
    ///
    /// Note that this implementation is purposefully slow. We would like you to think of ways to
    /// improve the performance!
    pub fn contains(&self, elem: &T) -> bool {
        let size = self.bitvector.size();

        let mut hasher = DefaultHasher::new();

        // Get the starting "seed" for hashing.
        elem.hash(&mut hasher);
        let mut hash = hasher.finish();

        for _ in 0..self.num_hashes {
            // Rehash the hash.
            hash.hash(&mut hasher);
            hash = hasher.finish();

            // Set the bit corresponding to this hash value.
            if !self.bitvector.get(hash as usize % size) {
                return false;
            }
        }

        true
    }
}
