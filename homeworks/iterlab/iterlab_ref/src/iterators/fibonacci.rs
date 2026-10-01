/// Represents an iterator that generates the Fibonacci sequence.
///
/// Think about what information you need to continuously and efficiently generate the next number
/// in the sequence? You will want to store that information in this struct.
#[derive(Clone)]
pub struct Fibonacci {
    curr: usize,
    next: usize,
}

impl Default for Fibonacci {
    /// Implement a default implementation for `Fibonacci`, where the sequence starts with 0 and 1.
    fn default() -> Self {
        Self::new(0, 1)
    }
}

impl Fibonacci {
    /// Create a new `Fibonacci` struct starting with the given `start` and `next` values.
    pub fn new(start: usize, next: usize) -> Self {
        Fibonacci { curr: start, next }
    }
}

/// Implement the `Iterator` trait for `Fibonacci`!
impl Iterator for Fibonacci {
    /// What should iterated type be?
    type Item = usize;

    /// Advances the iterator and returns the next Fibonacci number in the sequence.
    ///
    /// Ask yourself this: when should we return `None` instead of `Some(n)`?
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next += current;

        Some(current)
    }
}
