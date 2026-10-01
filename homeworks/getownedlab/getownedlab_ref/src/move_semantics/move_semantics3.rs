/// Refactor this code so that instead of passing `vec0` into the `fill_vec`
/// function, the vector gets created in the function itself and passed back to
/// the main function.

#[test]
fn vec_in_a_box() {
    let vec1 = fill_vec();

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
#[cfg(test)]
fn fill_vec() -> Vec<i32> {
    // Instead, let's create and fill the Vec in here - how do you do that?
    vec![22, 44, 66, 88]
}
