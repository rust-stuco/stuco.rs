/// Make me compile!
///
/// Hint: Use a tuple index to access the second element of `numbers`. You can put the
/// expression for the second element where ??? is so that the test passes.
/// This might be [helpful](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = ???;

    assert_eq!(2, second, "This is not the 2nd number in the tuple!");
}
