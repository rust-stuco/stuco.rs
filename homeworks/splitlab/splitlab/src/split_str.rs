/// A struct that represents split operations on a string.
///
/// TODO(student): You will need to change the `Split` struct to use lifetime parameter(s).
#[derive(Debug)]
pub struct Split {
    /// The remainder of the string that has not yet been split.
    ///
    /// Before the iterator has yielded any substrings, this is the entire string.
    /// After each call to `next`, this is the part of the string that has not yet been split.
    ///
    /// TODO(student): Replace the `'static` lifetime with something else!
    remainder: Option<&'static str>,

    /// The delimiter used to split the haystack string.
    ///
    /// TODO(student): Replace the `'static` lifetime with something else!
    delimiter: &'static str,
}

impl Split {
    /// Creates a new `Split` instance with the given haystack and delimiter.
    ///
    /// Should panic if the delimiter is empty (length 0).
    ///
    /// TODO(student): Replace the `'static` lifetimes with other lifetimes!
    pub fn new(haystack: &'static str, delimiter: &'static str) -> Self {

        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

// Note that we don't need to specify the `'delimiter` lifetime parameter because we never actually
// "use" it, we just need to carry it around to show that is just some "different" lifetime than the
// `'haystack` lifetime.
impl Iterator for Split {
    /// This iterator yields substrings of the original `haystack` string, split by some delimiter.
    ///
    /// TODO(student): Replace the `'static` lifetime with something else!
    type Item = &'static str;

    /// Returns the next substring of the original `haystack` string, split by some delimiter.
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement me (make sure to fix the lifetimes!)")
    }
}
