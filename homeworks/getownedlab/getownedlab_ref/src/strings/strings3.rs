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
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
