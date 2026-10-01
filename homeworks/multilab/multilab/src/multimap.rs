use std::collections::HashMap;
use std::hash::Hash;

/// A multimap that associates keys with multiple values.
///
/// This structure is similar to a [`HashMap`], but it allows multiple values to be
/// associated with a single key, instead of the usual one-to-one mapping.
/// It is implemented using a [`HashMap`] where each
/// value is a [`Vec`] of the associated values.
///
/// # Examples
///
/// ```rust
/// # use multilab::multimap::MultiMap;
/// #
/// let mut multimap = MultiMap::new();
/// multimap.insert(1, "hello");
/// multimap.insert(1, "world");
/// multimap.insert(2, "foo");
///
/// assert_eq!(multimap.get_values(&1).unwrap(), &["hello", "world"]);
/// assert_eq!(multimap.get_values(&2).unwrap(), &["foo"]);
/// ```
///
/// ### `Eq + Hash`
/// See the note in the documentation for [`MultiSet`](crate::multiset::MultiSet).
///
/// ### Public Field
///
/// Note that unlike [`MultiSet`](crate::multiset::MultiSet), it is fine to expose the
/// inner hash table here because we are not storing any other metadata
/// (in the previous section, you probably had more than one field tracking data).
/// This means that an outsider can't mess up our data structure by breaking any invariants.
/// So this type is really just a wrapper around the inner data structure
/// that provides some useful methods.
///
/// Also, we wanted to expose it to help with our test cases 😊.
#[derive(Debug)]
pub struct MultiMap<K: Hash + Eq, V: Eq> {
    /// The internal [`HashMap`] storing the key-value associations.
    pub inner: HashMap<K, Vec<V>>,
}

impl<K: Hash + Eq, V: Eq> MultiMap<K, V> {
    /// Creates a new, empty [`MultiMap`].
    pub fn new() -> Self {
        todo!()
    }

    /// Inserts a key-value pair into the [`MultiMap`].
    ///
    /// If the key already exists, the value is added to the existing vector of values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use multilab::multimap::MultiMap;
    /// #
    /// let mut multimap = MultiMap::new();
    /// multimap.insert(1, "hello");
    /// multimap.insert(1, "world");
    ///
    /// assert_eq!(multimap.get_values(&1).unwrap(), &["hello", "world"]);
    /// ```
    pub fn insert(&mut self, key: K, value: V) {
        todo!()
    }

    /// Removes a specific value from the values associated with a key.
    ///
    /// Returns `true` if the value was removed, `false` if the key or value was not found.
    ///
    /// Note that removing all values of a key will remove the key itself and the
    /// `Vec<V>` mapped to it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use multilab::multimap::MultiMap;
    /// #
    /// let mut multimap: MultiMap<i32, &'static str> = MultiMap::new();
    /// multimap.insert(1, "hello");
    /// multimap.insert(1, "world");
    ///
    /// assert!(multimap.remove_value(&1, &"hello"));
    /// assert_eq!(multimap.get_values(&1).unwrap(), &["world"]);
    /// ```
    #[allow(clippy::needless_range_loop)]
    pub fn remove_value(&mut self, key: &K, value: &V) -> bool {
        todo!()
    }

    /// Removes all values associated with a key and returns them.
    ///
    /// Returns `None` if the key was not found.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use multilab::multimap::MultiMap;
    /// #
    /// let mut multimap = MultiMap::new();
    /// multimap.insert(1, "hello");
    /// multimap.insert(1, "world");
    ///
    /// assert_eq!(multimap.remove_key(&1).unwrap(), &["hello", "world"]);
    /// assert!(multimap.get_values(&1).is_none());
    /// ```
    pub fn remove_key(&mut self, key: &K) -> Option<Vec<V>> {
        todo!()
    }

    /// Gets the values associated with a key, if any.
    ///
    /// Returns a reference to the vector of values if the key exists, otherwise `None`.
    /// The returned reference is immutable and can't be modified.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use multilab::multimap::MultiMap;
    /// #
    /// let mut multimap = MultiMap::new();
    /// multimap.insert(1, "hello");
    ///
    /// assert_eq!(multimap.get_values(&1).unwrap(), &["hello"]);
    /// ```
    ///
    /// # Challenge
    ///
    /// As an exercise, see if you can convert the return type to be `Option<&[V]>` instead.
    /// All of the tests cases should still be able to compile!
    pub fn get_values(&self, key: &K) -> Option<&[V]> {
        todo!()
    }

    /// Gets a mutable reference to the values associated with a key, if any.
    ///
    /// Returns a mutable reference to the vector of values if the key exists, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use multilab::multimap::MultiMap;
    /// #
    /// let mut multimap = MultiMap::new();
    /// multimap.insert(1, "hello");
    ///
    /// if let Some(values) = multimap.get_values_mut(&1) {
    ///     values.push("world");
    /// }
    ///
    /// assert_eq!(multimap.get_values(&1).unwrap(), &["hello", "world"]);
    /// ```
    pub fn get_values_mut(&mut self, key: &K) -> Option<&mut Vec<V>> {
        todo!()
    }
}

impl<K, V> Default for MultiMap<K, V>
where
    K: Eq + Hash,
    V: Eq,
{
    fn default() -> Self {
        Self::new()
    }
}
