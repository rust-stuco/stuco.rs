use std::fmt;
use std::fmt::Debug;

/// A vector that holds values of any type that implements [`Debug`].
#[derive(Default)]
pub struct DebugVec {
    // TODO(phase 2): Add field(s) and parameter(s)
}

impl DebugVec {
    /// Constructs a new, empty `DebugVec`.
    ///
    /// # Examples
    ///
    /// ```
    /// use bomblab::DebugVec;
    ///
    /// let vec = DebugVec::new();
    /// assert_eq!(format!("{vec:?}"), "[]");
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends an element that implements [`Debug`] to the back of a collection.
    ///
    /// # Examples
    ///
    /// ```
    /// use bomblab::DebugVec;
    ///
    /// let mut vec = DebugVec::new();
    /// vec.push(98008);
    /// vec.push("ferris");
    /// vec.push(true);
    /// assert_eq!(format!("{vec:?}"), "[98008, \"ferris\", true]");
    /// ```
    pub fn push<T: Debug>(&mut self, value: T) {
        // TODO(phase 2): Feel free to change the bounds on T, as long as the tests pass
        todo!("implement me for phase 2");
    }
}

impl Debug for DebugVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO(phase 2): Uncomment the line below and add your iterator
        // f.debug_list().entries(/* YOUR ITERATOR HERE */).finish()
        todo!("implement me for phase 2");
    }
}
