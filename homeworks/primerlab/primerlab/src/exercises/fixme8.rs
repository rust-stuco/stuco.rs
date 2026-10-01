/// Make me compile!
///
/// Hint: Create an array with at least 100 elements in it where the ??? is.
/// This might be [helpful](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

#[test]
fn big_arr() {
    let a = ???;

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
