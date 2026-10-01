// Define interface for bit vector
// My reason is to separate out the bit vector interface from its
// implementation, especially since I have multiple private implementations
// of the same method in my BitVector struct.

#[allow(dead_code)]
pub trait IBitVector {
    fn get(&self, idx: usize) -> bool;
    fn set(&mut self, idx: usize, val: bool);
    fn set_batch(&mut self, count: usize, idx: [u64; 8], val: bool);
    fn size(&self) -> usize;
}

// Our implementation of BitVector
#[derive(Debug, Clone)]
pub struct BitVector {
    bytes: Vec<u8>, // ordered from LSB to MSB
}

impl BitVector {
    // Parameters
    //  * size: number of bits to support
    pub fn new(size: usize) -> Self {
        BitVector {
            // Align to 32 bytes for SIMD operations
            bytes: vec![0; (size.div_ceil(8) + 31) & !31],
        }
    }

    // Oh yes, I implemented three versions of the `set` operation

    fn _set_v1(&mut self, chunk: usize, offset: usize, val: bool) {
        // "Keep it simple, silly"
        let mask = 1 << offset;
        if val {
            self.bytes[chunk] |= mask
        } else {
            self.bytes[chunk] &= !mask
        }
    }

    fn _set_v2(&mut self, chunk: usize, offset: usize, val: bool) {
        // "We don't want branching"
        let prev = self.bytes[chunk] as i32;
        self.bytes[chunk] = (prev ^ ((-(val as i32) ^ prev) & (1 << offset))) as u8
    }

    fn _set_v3(&mut self, chunk: usize, offset: usize, val: bool) {
        self.bytes[chunk] =
            // (1) Zero out the desired bit 
            (self.bytes[chunk] & !(1 << offset))
            // (2) Set back to 1 if setting to 1, leave as zero otherwise
            | ((val as u8) << offset);
    }
}

impl IBitVector for BitVector {
    fn get(&self, idx: usize) -> bool {
        if idx >= self.bytes.len() * 8 {
            return false;
        }
        let chunk = idx.wrapping_div(8);
        let offset = idx.wrapping_rem(8);
        ((self.bytes[chunk] >> offset) & 1) == 1
    }

    fn set(&mut self, idx: usize, val: bool) {
        if idx >= self.bytes.len() << 3 {
            return;
        }

        let chunk = idx.wrapping_div(8);
        let offset = idx.wrapping_rem(8);

        self._set_v3(chunk, offset, val);
    }
    fn set_batch(&mut self, count: usize, idx: [u64; 8], val: bool) {
        #[allow(clippy::needless_range_loop)]
        for i in 0..count {
            self.set(idx[i] as usize, val);
        }
    }
    fn size(&self) -> usize {
        self.bytes.len() * 8
    }
}
