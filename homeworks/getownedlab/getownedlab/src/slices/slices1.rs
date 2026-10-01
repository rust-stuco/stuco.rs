/// Make me compile!
/// Hint: What does the compiler say?

#[test]
#[allow(unused_variables)]
fn slice_me() {
    let arr = [1, 2, 3];
    let s1: [i32] = arr[0..2];

    let s2: str = "hello, world" as str;

    println!("Success!");
}
