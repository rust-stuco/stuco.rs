/// Make me compile by ONLY reordering the lines in `main()`,
/// but without adding, changing or removing any of them.
/// Hint: You only need to reorder 1 line.

#[test]
fn get_in_line() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
