use crate::{Bomb, DebugVec, defuser::defuse};

// Reminder: You may not edit this file!

#[test]
fn test_debug_vec_simple() {
    let mut vec = DebugVec::new();
    assert_eq!(format!("{vec:?}"), "[]");

    #[derive(Debug)]
    struct Unit;

    vec.push(98008);
    vec.push("ferris");
    vec.push(Vec::from([9, 8, 0, 0, 8]));
    vec.push(Unit);

    assert_eq!(
        format!("{vec:?}"),
        "[98008, \"ferris\", [9, 8, 0, 0, 8], Unit]"
    );
}

#[test]
fn test_debug_vec_other_formats() {
    let mut vec = DebugVec::new();

    vec.push(255);
    assert_eq!(format!("{vec:x?}"), "[ff]");
    assert_eq!(format!("{vec:X?}"), "[FF]");

    vec.push(true);
    vec.push(('h', 'i'));
    assert_eq!(
        format!("{vec:#?}"),
        "[\n    255,\n    true,\n    (\n        'h',\n        'i',\n    ),\n]"
    );
}

#[test]
fn test_debug_vec_flexible_lifetime_bounds() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Container<'a>(&'a str);

    let name = String::from("ferris");
    let mut vec = DebugVec::new();

    // If this is erroring, you may have used 'static (Why doesn't that work?)
    // You probably do NOT want to follow the compiler's hints in this case!
    vec.push(Container(&name));

    assert_eq!(format!("{vec:?}"), "[Container(\"ferris\")]");
}

#[test]
fn test_defused_bomb_does_not_explode() {
    let bomb = Bomb;
    defuse(bomb);
}
