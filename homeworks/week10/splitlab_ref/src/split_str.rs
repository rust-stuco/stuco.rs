/// A struct that represents split operations on a string.
#[derive(Debug)]
pub struct Split<'haystack, 'delimiter> {
    /// The remainder of the string that has not yet been split.
    ///
    /// Before the iterator has yielded any substrings, this is the entire string.
    /// After each call to `next`, this is the part of the string that has not yet been split.
    remainder: Option<&'haystack str>,

    /// The delimiter used to split the haystack string.
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> Split<'haystack, 'delimiter> {
    /// Creates a new `Split` instance with the given haystack and delimiter.
    ///
    /// Should panic if the delimiter is empty (length 0).
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        if delimiter.is_empty() {
            panic!("Delimiter cannot be empty")
        }

        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// Note that we don't need to specify the `'delimiter` lifetime parameter because we never actually
// "use" it, we just need to carry it around to show that is just some "different" lifetime than the
// `'haystack` lifetime.
impl<'haystack> Iterator for Split<'haystack, '_> {
    /// This iterator yields substrings of the original `haystack` string, split by some delimiter.
    type Item = &'haystack str;

    /// Returns the next substring of the original `haystack` string, split by some delimiter.
    ///
    fn next(&mut self) -> Option<Self::Item> {
        // If `remainder` is `None`, then there is nothing left to yield, and we should return
        // `None` immediately (with `?`).
        let remainder = self.remainder?;

        // Find the position of the delimiter in the remainder, if it exists.
        let Some(index) = remainder.find(self.delimiter) else {
            // If there is no delimiter in the remainder, return the entire remainder.
            return self.remainder.take();
        };

        let (start, end) = (index, index + self.delimiter.len());

        // Compute the next string to yield as well as the new remainder.
        let next_str = &remainder[..start];
        let new_remainder = &remainder[end..];

        // Replace the old remainder with the new remainder.
        let old_remainder = self.remainder.replace(new_remainder);
        assert!(
            old_remainder.is_some(),
            "We somehow found a delimiter in an empty remainder"
        );

        Some(next_str)
    }
}
