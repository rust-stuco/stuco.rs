/// A struct that represents split operations on a string.
pub struct SplitPattern<'haystack, P> {
    /// The remainder of the string that has not yet been split.
    ///
    /// Before the iterator has yielded any substrings, this is the entire string.
    /// After each call to `next`, this is the part of the string that has not yet been split.
    remainder: Option<&'haystack str>,

    /// The generic delimiter pattern used to split the haystack string.
    delimiter: P,
}

impl<'haystack, P> SplitPattern<'haystack, P> {
    /// Creates a new `Split` instance with the given haystack and delimiter.
    pub fn new(haystack: &'haystack str, delimiter: P) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, P> Iterator for SplitPattern<'haystack, P>
where
    P: Pattern,
{
    /// This iterator yields substrings of the original `haystack` string, split by some delimiter
    /// pattern.
    type Item = &'haystack str;

    /// Returns the next substring of the original `haystack` string, split by some delimiter
    /// pattern.
    ///
    /// Should panic if the delimiter is empty (length 0).
    fn next(&mut self) -> Option<Self::Item> {
        // If `remainder` is `None`, then there is nothing left to yield, and we should return
        // `None` immediately (with `?`).
        let remainder = self.remainder?;

        // Find the position of the delimiter in the remainder, if it exists.
        let Some((start, end)) = self.delimiter.find_next(remainder) else {
            // If there is no delimiter in the remainder, return the entire remainder.
            return self.remainder.take();
        };
        if start == end {
            panic!("Delimiter cannot be empty");
        }

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

/// An interface for a type that can find itself in a string.
pub trait Pattern {
    /// Finds the next occurrence of the pattern in the given string.
    ///
    /// Returns `Some((start, end))` if the pattern is found, where `start` is starting index of the
    /// pattern and `end` is the index of the end of the pattern in the string.
    /// Returns `None` if the pattern is not found.
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl Pattern for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

impl Pattern for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        // Remember that characters are not always one byte long! Make sure to use `len_utf8`.
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
    }
}

impl<F> Pattern for F
where
    F: Fn(char) -> bool,
{
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| self(*c))
            .map(|(start, c)| (start, start + c.len_utf8()))
    }
}

impl Pattern for [char; 1] {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        self[0].find_next(s)
    }
}

impl Pattern for &[char] {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        if self.is_empty() {
            return Some((0, 0));
        }

        s.char_indices()
            .find(|(_, c)| self.contains(c))
            .map(|(start, c)| (start, start + c.len_utf8()))
    }
}
