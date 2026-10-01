/// Make me compile!
///
/// Hint: Destructure the `cat` tuple so that the println will work.
/// This might be [helpful](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types)

#[test]
fn old_cats() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
