use std::collections::HashMap;
use std::hash::Hash;

/// A multiset that can store elements of type `K`.
///
/// Multisets, also known as bags, are collections that allow for duplicate elements.
/// This type efficiently tracks the multiplicity of each element.
///
/// # Examples
///
/// ```
/// # use multilab_ref::multiset::MultiSet;
/// #
/// let mut words = MultiSet::new();
/// words.insert("hello");
/// words.insert("world");
/// words.insert("hello");
///
/// assert_eq!(words.count(&"hello"), 2);  // "hello" appears twice
/// assert_eq!(words.count(&"world"), 1);
/// ```
///
/// You will need to decide what private fields your type should have. We would recommend
/// looking at the [`std::collections`] module and see if there are any collections there that
/// could be helpful for this problem!
/// Once you have an idea, replace the `_replace_me` with your own fields!
///
/// ### `Eq + Hash`
/// For now, ignore the `Eq + Hash` annotation next to the generic. These are _traits_, and
/// we will talk about them next week. This annotation is saying that we must be able to
/// check equality between two values of type `K`, and that `K` must have some
/// hash method that allows us to hash an element of type `K`.
/// _This may or may not be useful for the inner data structure that you choose..._
///
/// This shouldn't have any effect on your implementation,
/// but if you run into trouble with this, please let us know!
pub struct MultiSet<K: Eq + Hash> {
    inner: HashMap<K, usize>,
    size: usize,
}

impl<K> MultiSet<K>
where
    K: Eq + Hash,
{
    /// Creates a new empty [`MultiSet`].
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            size: 0,
        }
    }

    /// Checks if a [`MultiSet`] is empty.
    ///
    /// # Examples
    ///
    /// A new empty [`MultiSet`] with 0 total elements:
    ///
    /// ```
    /// # use multilab_ref::multiset::MultiSet;
    /// #
    /// let multiset: MultiSet<char> = MultiSet::new();
    /// assert_eq!(0, multiset.len());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Counts all the elements, including each duplicate.
    ///
    /// # Examples
    ///
    /// A [`MultiSet`] after insering 1, 2, and 1 has 3 total elements:
    ///
    /// ```
    /// # use multilab_ref::multiset::MultiSet;
    /// #
    /// let mut multiset = MultiSet::new();
    /// multiset.insert(1);
    /// multiset.insert(2);
    /// multiset.insert(1);
    /// assert_eq!(multiset.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.size
    }

    /// Checks if a given value is in the [`MultiSet`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use multilab_ref::multiset::MultiSet;
    /// #
    /// let mut multiset = MultiSet::new();
    /// multiset.insert(1);
    /// multiset.insert(2);
    /// multiset.insert(1);
    /// assert!(multiset.contains(&1));
    /// assert!(multiset.contains(&2));
    /// assert!(!multiset.contains(&3));
    /// ```
    pub fn contains(&self, value: &K) -> bool {
        self.inner.contains_key(value)
    }

    /// Inserts an element.
    ///
    /// # Examples
    ///
    /// Insert `5` into a new [`MultiSet`]:
    ///
    /// ```
    /// use multilab_ref::multiset::MultiSet;
    ///
    /// let mut multiset: MultiSet<i32> = MultiSet::new();
    /// assert_eq!(0, multiset.count(&5));
    /// multiset.insert(5);
    /// assert_eq!(1, multiset.count(&5));
    /// ```
    pub fn insert(&mut self, value: K) {
        self.size += 1;
        *self.inner.entry(value).or_insert(0) += 1;
    }

    /// Removes an element.
    ///
    /// If the element does not exist in the [`MultiSet`],
    /// returns false. Otherwise, it removes and returns true.
    ///
    /// # Examples
    ///
    /// Remove `5` from a new [`MultiSet`]:
    ///
    /// ```
    /// # use multilab_ref::multiset::MultiSet;
    /// #
    /// let mut multiset: MultiSet<i32> = MultiSet::new();
    /// multiset.insert(5);
    /// assert_eq!(1, multiset.count(&5));
    /// assert!(multiset.remove(&5));
    /// assert_eq!(0, multiset.count(&5));
    /// assert!(!multiset.remove(&5));
    /// ```
    pub fn remove(&mut self, value: &K) -> bool {
        match self.inner.get_mut(value) {
            None => false,
            Some(count) => {
                debug_assert!(*count > 0);
                if *count == 1 {
                    self.inner.remove(value);
                } else {
                    *count -= 1;
                }
                self.size -= 1;
                true
            }
        }
    }

    /// Counts the occurrences of `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use multilab_ref::multiset::MultiSet;
    /// #
    /// let mut multiset: MultiSet<u8> = MultiSet::new();
    /// multiset.insert(0);
    /// multiset.insert(0);
    /// multiset.insert(1);
    /// multiset.insert(0);
    /// assert_eq!(3, multiset.count(&0));
    /// assert_eq!(1, multiset.count(&1));
    /// ```
    pub fn count(&self, value: &K) -> usize {
        self.inner.get(value).map_or(0, |x| *x)
    }
}

impl<K> Default for MultiSet<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
