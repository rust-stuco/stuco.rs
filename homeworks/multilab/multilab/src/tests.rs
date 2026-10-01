mod multiset_tests {
    use crate::multiset::MultiSet;

    #[test]
    fn test_insert_remove_basic() {
        let mut multiset: MultiSet<char> = MultiSet::new();
        assert_eq!(0, multiset.count(&'a'));
        assert!(!multiset.remove(&'a'));

        let multiset: MultiSet<String> = MultiSet::new();
        assert_eq!(0, multiset.count(&"hello".to_string()));

        let mut multiset: MultiSet<i32> = MultiSet::new();
        multiset.insert(5);
        multiset.insert(5);
        assert_eq!(2, multiset.len());
        assert_eq!(2, multiset.count(&5));

        let mut multiset: MultiSet<i32> = MultiSet::new();
        multiset.insert(5);
        multiset.insert(3);
        multiset.insert(5);
        assert_eq!(2, multiset.count(&5));
        assert_eq!(1, multiset.count(&3));
        assert_eq!(0, multiset.count(&10));
    }

    #[test]
    fn test_insert_remove_more() {
        let mut multiset: MultiSet<i32> = MultiSet::new();
        multiset.insert(5);
        assert_eq!(1, multiset.len());
        assert!(multiset.contains(&5));
        assert!(multiset.remove(&5));
        assert!(multiset.is_empty());

        let mut multiset: MultiSet<i32> = MultiSet::new();
        multiset.insert(5);
        multiset.insert(5);
        assert_eq!(2, multiset.len());
        assert!(multiset.remove(&5));
        assert_eq!(1, multiset.len());
        assert_eq!(1, multiset.count(&5));

        let mut multiset: MultiSet<i32> = MultiSet::new();
        multiset.insert(5);
        multiset.insert(3);
        multiset.insert(5);
        assert_eq!(3, multiset.len());
        assert!(multiset.remove(&5));
        assert_eq!(2, multiset.len());
        assert_eq!(1, multiset.count(&5));
        assert!(multiset.remove(&3));
        assert_eq!(1, multiset.len());
        assert!(multiset.remove(&5));
        assert!(multiset.is_empty());
    }

    #[test]
    fn test_multiple_insertions_and_removals() {
        let mut multiset: MultiSet<String> = MultiSet::new();

        // Insert a variety of elements, some duplicates
        multiset.insert("apple".to_string());
        multiset.insert("banana".to_string());
        multiset.insert("apple".to_string());
        multiset.insert("cherry".to_string());
        multiset.insert("banana".to_string());

        assert_eq!(5, multiset.len());
        assert_eq!(2, multiset.count(&"apple".to_string()));
        assert_eq!(2, multiset.count(&"banana".to_string()));
        assert_eq!(1, multiset.count(&"cherry".to_string()));

        // Remove some elements, including duplicates
        assert!(multiset.remove(&"apple".to_string()));
        assert!(multiset.remove(&"banana".to_string()));
        assert!(multiset.remove(&"cherry".to_string()));

        assert_eq!(2, multiset.len());
        assert_eq!(1, multiset.count(&"apple".to_string()));
        assert_eq!(1, multiset.count(&"banana".to_string()));

        // Remove the remaining elements
        assert!(multiset.remove(&"apple".to_string()));
        assert!(multiset.remove(&"banana".to_string()));

        assert!(multiset.is_empty());
    }

    #[test]
    fn test_interleaved_insert_remove() {
        let mut multiset: MultiSet<i32> = MultiSet::new();

        let mut removal_counter = 0;

        for i in 0..10_000 {
            multiset.insert(i % 100);

            if i % 2 == 0 {
                multiset.remove(&(removal_counter % 100));
                removal_counter += 1;
            }
        }

        assert_eq!(5_000, multiset.len());
        assert_eq!(50, multiset.count(&0));
    }
}

mod multimap_tests {
    use crate::multimap::MultiMap;

    #[test]
    fn test_insert_duplicate_values() {
        let mut multimap = MultiMap::new();
        multimap.insert(1, "hello");
        multimap.insert(1, "hello");

        assert_eq!(multimap.get_values(&1).unwrap(), &["hello", "hello"]);
    }

    #[test]
    fn test_insert_many_values() {
        let mut multimap = MultiMap::new();
        for i in 0..1000 {
            multimap.insert(1, i);
        }

        assert_eq!(multimap.get_values(&1).unwrap().len(), 1000);
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut multimap: MultiMap<i32, Option<()>> = MultiMap::new();
        assert_eq!(multimap.get_values(&1), None);
        assert_eq!(multimap.remove_key(&1), None);
    }

    #[test]
    fn test_remove_value_from_multiple() {
        let mut multimap = MultiMap::new();

        multimap.insert(1, "hello");
        assert!(multimap.remove_value(&1, &"hello"));
        assert!(multimap.get_values(&1).is_none());

        multimap.insert(1, "hello");
        multimap.insert(1, "world");
        assert!(multimap.remove_value(&1, &"hello"));
        assert_eq!(multimap.get_values(&1).unwrap(), &["world"]);
    }

    #[test]
    fn test_large_insert_remove() {
        let mut multimap = MultiMap::new();

        // Insert 100 values for 1000 keys
        for i in 0..1000 {
            for j in 0..100 {
                multimap.insert(i, j);
            }
        }

        // Assert the total number of key-value pairs (indirectly)
        let mut total_count = 0;
        for (_, values) in multimap.inner.iter() {
            total_count += values.len();
        }
        assert_eq!(total_count, 100000);

        // Remove half of the values for each key using indices
        for i in 0..1000 {
            if let Some(values) = multimap.get_values_mut(&i) {
                let mut remaining_count = values.len();
                for _ in 0..(remaining_count / 2) {
                    let index_to_remove = 0; // Remove from the beginning for simplicity
                    values.remove(index_to_remove);
                    remaining_count -= 1;
                }
                assert_eq!(remaining_count, 50);
            }
        }

        // Assert the remaining number of key-value pairs (indirectly)
        total_count = 0;
        for (_, values) in multimap.inner.iter() {
            total_count += values.len();
        }
        assert_eq!(total_count, 50000);
    }
}
