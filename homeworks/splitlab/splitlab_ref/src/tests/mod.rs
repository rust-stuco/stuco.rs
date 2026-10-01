use crate::split_pattern::SplitPattern;
use crate::split_str::Split;

/// A macro that tests [`Split`] for correctness.
macro_rules! test_str {
    ($name:ident, $haystack:expr, $pattern:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let haystack = String::from($haystack);

            // Test with `Split`.
            let split_result = {
                let pattern = String::from($pattern);
                Split::new(&haystack, &pattern).collect::<Vec<&str>>()
            };

            assert_eq!(split_result, $expected);

            // Test against the real `split` method.
            assert_eq!(
                split_result,
                haystack.split($pattern).collect::<Vec<&str>>()
            );
        }
    };
}

/// A macro that tests [`SplitPattern`] for correctness.
macro_rules! test_pattern {
    ($name:ident, $haystack:expr, $pattern:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let haystack = String::from($haystack);

            // Test with `SplitPattern`.
            let pattern_result = SplitPattern::new(&haystack, $pattern).collect::<Vec<&str>>();

            assert_eq!(pattern_result, $expected);

            // Test against the real `split` method.
            assert_eq!(
                pattern_result,
                haystack.split($pattern).collect::<Vec<&str>>()
            );
        }
    };
}

mod pattern;
mod str;
