/// A simple bit vector backed by a [`Vec<u8>`].
#[derive(Debug, Clone, Default)]
pub struct BitVector {
    /// We store the bit vector as a vector of bytes.
    inner: Vec<u8>,
}

impl BitVector {
    /// Creates a new bit vector with a minimum of `num_bits` number of bits.
    ///
    /// Note that `num_bits` will be rounded up to a multiple of 8.
    pub fn new(num_bits: usize) -> Self {
        let bytes = (num_bits / 8) + 1; // Approximate round-up.
        let inner = vec![0; bytes];
        Self { inner }
    }

    /// Returns the number of bits that this bit vector manages.
    pub fn size(&self) -> usize {
        self.inner.len() * 8
    }

    /// Sets a bit at the given index to the given value (1 if `true`, 0 if `false`).
    pub fn set(&mut self, index: usize, value: bool) {
        let byte = index / 8;
        let bit = index % 8;

        // Set the bit to whatever the given value is.
        // Note that you can do all of this inline.
        let removed = self.inner[byte] & !(1u8 << bit);
        let with_value = (value as u8 & 1u8) << bit;

        self.inner[byte] = removed | with_value;
    }

    /// Retrieves a bit at the given index, returning `true` if it is 1 and `false` if it is 0.
    pub fn get(&self, index: usize) -> bool {
        let byte = index / 8;
        let bit = index % 8;

        // Check if the bit at the offset is turned on.
        (self.inner[byte] >> bit) & 1 == 1
    }
}
