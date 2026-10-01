/// Make me compile by only reordering the lines!

#[test]
fn wait_for_me_to_speak() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear();
}

#[cfg(test)]
fn first_word(s: &str) -> &str {
    &s[..1]
}
