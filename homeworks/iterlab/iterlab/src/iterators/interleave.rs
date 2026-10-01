/// Creates an iterator that interleaves elements from two provided iterators.
///
/// This iterator alternates between the first and second iterators until both are exhausted.
#[derive(Clone)]
pub struct Interleave<I, J> {
    todo1: std::marker::PhantomData<I>, // Replace me!
    todo2: std::marker::PhantomData<J>, // Replace me!
}

impl<I, J> Interleave<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    /// Creates a new `Interleave` iterator from the given two iterators.
    ///
    /// Both iterators must have the same element type.
    pub fn new(first_iter: I, second_iter: J) -> Self {
        todo!()
    }
}

/// Implement the `Iterator` trait for `Interleave`!
impl<I, J> Iterator for Interleave<I, J>
where
    I: Iterator,
    J: Iterator<Item = I::Item>, // Both iterators should yield the same type
{
    /// Output the same type as the input.
    type Item = <I as Iterator>::Item;

    /// Advances the iterator and returns the next interleaved element.
    ///
    /// This alternates between taking elements from the first and second iterators.
    /// If one iterator is exhausted, the remaining elements will be taken from the other iterator.
    fn next(&mut self) -> Option<<I as Iterator>::Item> {
        todo!()
    }
}
