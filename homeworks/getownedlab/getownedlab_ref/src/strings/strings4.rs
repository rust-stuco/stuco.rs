// Implement the 3 functions!

// Try not to just google the answer. The official Rust documentation will be helpful:

// - https://doc.rust-lang.org/std/primitive.str.html
// You can also just enter into the search bar and have the docs find things for you.

#[cfg(test)]
fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

#[cfg(test)]
fn compose_me(input: &str) -> String {
    input.to_string() + " world!"
}

#[cfg(test)]
fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool"
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons"
        );
    }
}
