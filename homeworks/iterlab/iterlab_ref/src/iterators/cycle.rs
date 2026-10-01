/// A struct that is intended to be an iterator that infinitely cycles through the elements of
/// another iterator.
#[derive(Clone)]
pub struct Cycle<I: Clone> {
    original: I,
    iter: I,
}

impl<I: Clone> Cycle<I> {
    /// Creates a new `Cycle` iterator from the given iterator.
    pub fn new(iter: I) -> Self {
        Self {
            original: iter.clone(),
            iter,
        }
    }
}

/// Implement the `Iterator` trait for `Cycle`!
impl<I: Clone + Iterator> Iterator for Cycle<I> {
    /// Output the same type as the input.
    type Item = <I as Iterator>::Item;

    /// Advances the iterator and returns the next item in the cycle.
    ///
    /// This will cycle back to the original iterator when the current iterator is exhausted.
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        match self.iter.next() {
            None => {
                self.iter = self.original.clone();
                self.iter.next()
            }
            item => item,
        }
    }
}
