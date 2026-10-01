use super::*;

#[test]
fn test_str_compiles() {
    let haystack = String::from("a b c d e");

    let results = {
        let delimeter = String::from(" ");

        let split_result: Vec<&str> = Split::new(&haystack, &delimeter).collect();

        split_result
    };

    assert_eq!(results, vec!["a", "b", "c", "d", "e"]);
}

#[test]
#[should_panic]
fn test_str_empty_delimiter() {
    let _ = Split::new(&String::from("a b c"), "");
}

test_str!(
    test_crust_of_rust_it_works,
    "a b c d e",
    " ",
    vec!["a", "b", "c", "d", "e"]
);

test_str!(
    test_crust_of_rust_tail,
    "a b c d ",
    " ",
    vec!["a", "b", "c", "d", ""]
);

test_str!(test_empty_haystack, "", " ", vec![""]);

test_str!(
    test_starting_delimiter,
    " a b c",
    " ",
    vec!["", "a", "b", "c"]
);

test_str!(test_long_delimiter, "a---b---c", "---", vec!["a", "b", "c"]);

test_str!(
    test_multiple_consecutive_delimiters,
    "a  b   c",
    " ",
    vec!["a", "", "b", "", "", "c"]
);

test_str!(test_no_delimiter_in_haystack, "abcde", "x", vec!["abcde"]);

test_str!(test_unicode_delimiter, "a😊b😊c", "😊", vec!["a", "b", "c"]);

test_str!(test_only_delimiters, ";;;", ";", vec!["", "", "", ""]);

test_str!(
    test_empty_haystack_multi_char_delimiter,
    "",
    "abc",
    vec![""]
);

test_str!(
    test_ending_delimiter,
    "a b c ",
    " ",
    vec!["a", "b", "c", ""]
);

test_str!(
    test_delimiters_with_spaces,
    "a, b, c",
    ", ",
    vec!["a", "b", "c"]
);

test_str!(
    test_mixed_delimiters,
    "a, b; c, d",
    ", ",
    vec!["a", "b; c", "d"]
);

test_str!(
    test_delimiter_not_in_ascii,
    "a→b→c",
    "→",
    vec!["a", "b", "c"]
);

test_str!(
    test_delimiter_with_escapes,
    "a\nb\nc",
    "\n",
    vec!["a", "b", "c"]
);

test_str!(
    test_haystack_with_bars,
    "a||b||c",
    "||",
    vec!["a", "b", "c"]
);
