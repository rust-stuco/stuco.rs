use super::*;

#[test]
#[should_panic]
fn test_pattern_empty_delimiter() {
    let _ = SplitPattern::new(&String::from("a b c"), "").collect::<Vec<&str>>();
}

test_pattern!(
    test_crust_of_rust_until_char_test,
    "hello world",
    'o',
    vec!["hell", " w", "rld"]
);

test_pattern!(test_pattern_char, "a,b,c", ',', vec!["a", "b", "c"]);

test_pattern!(
    test_pattern_str,
    "apple_banana_cherry",
    "_",
    vec!["apple", "banana", "cherry"]
);

test_pattern!(test_pattern_char_array, "a b c", [' '], vec!["a", "b", "c"]);

test_pattern!(
    test_pattern_char_slice,
    "a,b;c.d",
    &[',', ';', '.'] as &[char],
    vec!["a", "b", "c", "d"]
);

test_pattern!(
    test_pattern_fn,
    "a1b2c3",
    |c: char| c.is_numeric(),
    vec!["a", "b", "c", ""]
);

test_pattern!(test_pattern_empty_haystack_char, "", 'a', vec![""]);

test_pattern!(test_pattern_empty_haystack_str, "", "delimiter", vec![""]);

test_pattern!(
    test_pattern_empty_haystack_fn,
    "",
    |c: char| c.is_numeric(),
    vec![""]
);

test_pattern!(
    test_pattern_no_match_fn,
    "abc",
    |c: char| c.is_numeric(),
    vec!["abc"]
);

test_pattern!(
    test_pattern_multiple_matches_char,
    "aaa",
    'a',
    vec!["", "", "", ""]
);

test_pattern!(test_pattern_unicode_char, "café", 'é', vec!["caf", ""]);

test_pattern!(test_pattern_only_pattern_char, ",,", ',', vec!["", "", ""]);
