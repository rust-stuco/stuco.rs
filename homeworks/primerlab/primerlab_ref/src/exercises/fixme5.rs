/// Make me compile!
///
/// No hints this time :D

#[test]
fn youre_square() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

#[cfg(test)]
fn square(num: i32) -> i32 {
    num * num
}
