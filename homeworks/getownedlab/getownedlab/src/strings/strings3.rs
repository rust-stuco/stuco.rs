// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

// You could brute force this... but try to guess before compiling and see if you're right!

#[cfg(test)]
fn string_slice(arg: &str) {
    println!("{}", arg);
}

#[cfg(test)]
fn string(arg: String) {
    println!("{}", arg);
}

#[test]
fn am_i_str_or_string() {
    // Some examples:
    string_slice("");
    string(String::new());

    // Replace all of the `todo!()`s with `string_slice` or `string`!
    todo!("blue");
    todo!("red".to_string());
    todo!(String::from("hi"));
    todo!("rust is fun!".to_owned());
    todo!("nice weather".into());
    todo!(format!("Interpolation {}", "Station"));
    todo!(&String::from("abc")[0..1]);
    todo!("  hello there ".trim());
    todo!("Happy Monday!".to_string().replace("Mon", "Tues"));
    todo!("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
