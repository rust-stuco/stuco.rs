// Make me compile without changing the function signature!

#[test]
fn my_favorite_color() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

#[cfg(test)]
fn current_favorite_color() -> String {
    "blue".to_string()
}
