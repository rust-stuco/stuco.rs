use primerlab::functions::is_prime;
use std::io;

/// The main function is our entrypoint, just like in C.
/// In this case, we implemented a nice command line interface, using
/// is_prime from lib.rs
fn main() {
    println!("Test if a number is prime!");

    loop {
        println!("Please input your number.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered: {}", input);

        if is_prime(input) {
            println!("{} is a prime!\n", input);
        } else {
            println!("{} is not a prime\n", input);
        }
    }
}
