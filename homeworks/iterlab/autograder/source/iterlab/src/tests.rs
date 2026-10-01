mod fibonacci_tests {
    use crate::iterators::fibonacci::*;

    #[test]
    fn test_default_starts_with_0_and_1() {
        let mut fib = Fibonacci::default();
        assert_eq!(fib.next(), Some(0));
        assert_eq!(fib.next(), Some(1));
    }

    #[test]
    fn test_custom_start_values() {
        let mut fib = Fibonacci::new(4, 6);
        assert_eq!(fib.next(), Some(4));
        assert_eq!(fib.next(), Some(6));
        assert_eq!(fib.next(), Some(10));
    }

    #[test]
    fn test_iteration_sequence() {
        let fib = Fibonacci::default();
        assert_eq!(
            vec![0, 1, 1, 2, 3, 5, 8, 13],
            fib.take(8).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_never_finishes() {
        let mut fib = Fibonacci::default();
        // We can't go too high otherwise we'll overflow
        for _ in 0..91 {
            fib.next();
        }
        assert_ne!(fib.next(), None);
    }
}

mod cycle_tests {
    use crate::iterators::cycle::*;

    #[test]
    fn basic_cycling() {
        {
            let numbers = [1, 2, 3];
            let mut cycle = Cycle::new(numbers.iter().cloned());

            assert_eq!(cycle.next(), Some(1));
            assert_eq!(cycle.next(), Some(2));
            assert_eq!(cycle.next(), Some(3));
            assert_eq!(cycle.next(), Some(1)); // Cycle back to the beginning
            assert_eq!(cycle.next(), Some(2));
            assert_eq!(cycle.next(), Some(3));

            for i in 0..1000 {
                assert_eq!(cycle.next(), Some((i % 3) + 1))
            }
        }

        {
            let mut cycle = Cycle::new(std::iter::repeat(1));

            // Only run a limited number of iterations to avoid an infinite loop
            for _ in 0..10 {
                assert_eq!(cycle.next(), Some(1));
            }
        }
    }

    #[test]
    fn empty_cycle() {
        {
            let binding = Vec::<()>::new();
            let empty = binding.iter().cloned();
            let mut cycle = Cycle::new(empty);
            assert_eq!(cycle.next(), None); // Should always yield None
        }

        {
            let mut cycle = Cycle::new(std::iter::empty::<i32>());
            assert_eq!(cycle.next(), None);
        }

        {
            let numbers = [1, 2, 3];
            let mut original_iter = numbers.iter().cloned();
            original_iter.next();
            original_iter.next();
            original_iter.next(); // Consume all elements

            let mut cycle = Cycle::new(original_iter);

            assert_eq!(cycle.next(), None); // Should still be None after the cycle
        }
    }

    #[test]
    fn partially_consumed_cycle() {
        let numbers = [1, 2, 3];
        let mut original_iter = numbers.iter().cloned();
        assert_eq!(original_iter.next(), Some(1)); // Consume one item

        let mut cycle = Cycle::new(original_iter);

        assert_eq!(cycle.next(), Some(2)); // Should start from the remaining items
        assert_eq!(cycle.next(), Some(3));
        assert_eq!(cycle.next(), Some(2)); // Cycle back to remaining items
        assert_eq!(cycle.next(), Some(3));
        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(3));
    }

    #[test]
    fn cycle_composed() {
        {
            let numbers = [1, 2, 3, 4, 5];
            let mut cycle = Cycle::new(numbers.iter().cloned().filter(|x| x % 2 == 0));

            assert_eq!(cycle.next(), Some(2));
            assert_eq!(cycle.next(), Some(4));
            assert_eq!(cycle.next(), Some(2)); // Cycle back to the filtered elements
            assert_eq!(cycle.next(), Some(4));
            assert_eq!(cycle.next(), Some(2));
        }

        {
            let numbers = [1, 2, 3, 4, 5];
            let mut cycle = Cycle::new(numbers.chunks(2));

            assert_eq!(cycle.next(), Some(&[1, 2][..]));
            assert_eq!(cycle.next(), Some(&[3, 4][..]));
            assert_eq!(cycle.next(), Some(&[5][..])); // Last chunk might be smaller
            assert_eq!(cycle.next(), Some(&[1, 2][..])); // Cycle back to the beginning
        }
    }
    #[test]
    fn mutating_inner_iterator() {
        #[derive(Clone)]
        struct DoublingIterator {
            values: Vec<i32>,
            index: usize,
        }

        impl Iterator for DoublingIterator {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index < self.values.len() {
                    self.values[self.index] *= 2; // Mutate the value
                    self.index += 1;
                    Some(self.values[self.index - 1])
                } else {
                    None
                }
            }
        }

        let iterator = DoublingIterator {
            values: vec![1, 2, 3],
            index: 0,
        };

        let mut cycle = Cycle::new(iterator);

        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(4));
        assert_eq!(cycle.next(), Some(6));
        assert_eq!(cycle.next(), Some(2));
        assert_eq!(cycle.next(), Some(4));
        assert_eq!(cycle.next(), Some(6));
    }
}

mod interleave_tests {
    use crate::iterators::interleave::*;

    #[test]
    fn basic_interleaving() {
        let numbers1 = [1, 3, 5];
        let numbers2 = [2, 4, 6];
        let mut interleaved = Interleave::new(numbers1.iter().cloned(), numbers2.iter().cloned());

        assert_eq!(interleaved.next(), Some(1));
        assert_eq!(interleaved.next(), Some(2));
        assert_eq!(interleaved.next(), Some(3));
        assert_eq!(interleaved.next(), Some(4));
        assert_eq!(interleaved.next(), Some(5));
        assert_eq!(interleaved.next(), Some(6));
    }

    #[test]
    fn empty_iterators() {
        let empty_vec: Vec<()> = vec![];
        let empty1 = empty_vec.iter().cloned();
        let empty2 = empty_vec.iter().cloned();
        let mut interleaved = Interleave::new(empty1, empty2);

        assert_eq!(interleaved.next(), None); // Should always yield None
    }

    #[test]
    fn one_empty_iterator() {
        let numbers = [1, 2, 3];
        let empty_vec: Vec<i32> = vec![];
        let empty = empty_vec.iter().cloned();
        let mut interleaved = Interleave::new(numbers.iter().cloned(), empty);

        assert_eq!(interleaved.next(), Some(1));
        assert_eq!(interleaved.next(), Some(2));
        assert_eq!(interleaved.next(), Some(3));
        assert_eq!(interleaved.next(), None);
    }

    #[test]
    fn different_length_iterators() {
        let numbers1 = [1, 2];
        let numbers2 = [3, 4, 5];
        let mut interleaved = Interleave::new(numbers1.iter().cloned(), numbers2.iter().cloned());

        assert_eq!(interleaved.next(), Some(1));
        assert_eq!(interleaved.next(), Some(3));
        assert_eq!(interleaved.next(), Some(2));
        assert_eq!(interleaved.next(), Some(4));
        assert_eq!(interleaved.next(), Some(5));
        assert_eq!(interleaved.next(), None);
    }

    #[test]
    fn partially_consumed_iterators() {
        let numbers1 = [1, 2, 3];
        let mut original_iter1 = numbers1.iter().cloned();
        assert_eq!(original_iter1.next(), Some(1)); // Consume one item

        let numbers2 = [4, 5, 6];
        let mut interleaved = Interleave::new(original_iter1, numbers2.iter().cloned());

        assert_eq!(interleaved.next(), Some(2)); // Starts from the remaining items
        assert_eq!(interleaved.next(), Some(4));
        assert_eq!(interleaved.next(), Some(3));
        assert_eq!(interleaved.next(), Some(5));
        assert_eq!(interleaved.next(), Some(6));
    }

    #[test]
    fn large_iterators() {
        let numbers1: Vec<i32> = (1..1001).step_by(2).collect();
        let numbers2: Vec<i32> = (2..1002).step_by(2).collect();
        let mut interleaved = Interleave::new(numbers1.iter().cloned(), numbers2.iter().cloned());

        for i in 1..1001 {
            assert_eq!(interleaved.next(), Some(i));
        }

        assert_eq!(interleaved.next(), None);
    }
}

mod double_tests {
    use crate::iterators::double::*;

    #[test]
    fn double_values() {
        {
            let numbers = [1, 2, 3];
            let mut doubled = Double::new(numbers.iter().cloned());

            assert_eq!(doubled.next(), Some(1));
            assert_eq!(doubled.next(), Some(1));
            assert_eq!(doubled.next(), Some(2));
            assert_eq!(doubled.next(), Some(2));
            assert_eq!(doubled.next(), Some(3));
            assert_eq!(doubled.next(), Some(3));
        }

        {
            let mut doubled = Double::new(std::iter::repeat(42));

            for _ in 0..10 {
                assert_eq!(doubled.next(), Some(42));
                assert_eq!(doubled.next(), Some(42));
            }
        }
    }
    #[test]
    fn single_element_iterator() {
        let single_iter = vec![1].into_iter();
        let mut doubled = Double::new(single_iter.clone());
        let mut redoubled = Double::new(doubled.clone());

        assert_eq!(doubled.next(), Some(1));
        assert_eq!(doubled.next(), Some(1));
        assert_eq!(doubled.next(), None);

        assert_eq!(redoubled.next(), Some(1));
        assert_eq!(redoubled.next(), Some(1));
        assert_eq!(redoubled.next(), Some(1));
        assert_eq!(redoubled.next(), Some(1));
        assert_eq!(redoubled.next(), None);
    }
}

mod sum_squares_tests {
    use crate::hofs::sum_squares::*;

    #[test]
    fn test_zero_largest_square_returns_zero() {
        assert_eq!(0, sum_of_squared_odd_numbers(0));
    }

    #[test]
    fn test_small_largest_square_returns_correct_sum() {
        assert_eq!(1, sum_of_squared_odd_numbers(1));
        assert_eq!(35, sum_of_squared_odd_numbers(30));
    }

    #[test]
    fn test_functional_and_imperative_versions_match() {
        for n in 0..1000 {
            let functional_result = sum_of_squared_odd_numbers(n);
            let imperative_result = sum_of_squared_odd_numbers_bad(n);
            assert_eq!(functional_result, imperative_result);
        }
    }
}

mod fib_fun_tests {
    use crate::hofs::fib_fun::*;

    #[test]
    fn test_sum_fib_range_empty_range() {
        assert_eq!(0, sum_fib_range(0, 0));
    }

    #[test]
    fn test_sum_fib_range_small_range() {
        assert_eq!(9959, sum_fib_range(15, 20));
    }

    #[test]
    fn test_sum_fib_range_large_range() {
        assert_eq!(1346266, sum_fib_range(3, 30));
    }

    #[test]
    fn test_read_the_docs_zero_input() {
        assert_eq!(Vec::<usize>::new(), read_the_docs(0));
    }

    #[test]
    fn test_read_the_docs_small_input() {
        assert_eq!(vec![0, 4, 8, 48], read_the_docs(6));
    }

    #[test]
    fn test_read_the_docs_larger_input() {
        assert_eq!(vec![0, 4, 8, 48, 100, 217, 486], read_the_docs(9));
    }

    #[test]
    fn test_read_the_docs_largest() {
        assert_eq!(
            vec![
                0, 4, 8, 48, 100, 217, 486, 1106, 6728, 15708, 37088, 88381, 212268, 513222,
                1248028, 3050113
            ],
            read_the_docs(20)
        );
    }
}
