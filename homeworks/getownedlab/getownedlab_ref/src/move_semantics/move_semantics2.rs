/// Make me compile by only adding a handful of characters!
/// Hint: What does the compiler tell you?
/// You also might want to remove something to get rid of the warning.

#[test]
fn youre_different() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0.clone());

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

#[cfg(test)]
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
