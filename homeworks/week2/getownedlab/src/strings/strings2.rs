// Make me compile without changing the function signature!

#[test]
fn whats_is_a_color() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

#[cfg(test)]
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
