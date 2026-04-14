#![allow(unused_variables)]

/// Solutions and some test cases adapted from [15-112](
/// https://www.kosbie.net/cmu/spring-22/15-112/notes/notes-loops.html#nthPrime).
///
/// Test cases are located in tests.rs!
/// Make sure this works!
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/// Given a number n, return true if it is a prime number, and false otherwise.
///
/// # Example
///
/// ```
/// use primerlab::functions::is_prime;
///
/// let test_prime = 2;
/// assert!(is_prime(test_prime));
///
/// let test_not_prime = 42;
/// assert!(!is_prime(test_not_prime));
/// ```
///
/// # Note
///
/// `isPrime` must be efficient. Refer to `fasterIsPrime` from [15-112](
/// https://www.kosbie.net/cmu/spring-22/15-112/notes/notes-loops.html#:~:text=fasterIsPrime).
pub fn is_prime(n: usize) -> bool {
    todo!()
}

/// Given a number n, return the nth prime. Refer to the test cases below for more details.
///
/// For example, the 0th prime is 2, and the 1st prime is 3, then the 2nd prime is 5, etc.
///
/// # Example
///
/// ```
/// use primerlab::functions::nth_prime;
///
/// let n = 4;
/// assert_eq!(nth_prime(n), 11);
///
/// let n = 20;
/// assert_eq!(nth_prime(n), 73);
/// ```
///
/// ### Suggestions
///
/// You can look [here](https://en.wikipedia.org/wiki/List_of_prime_numbers) for more primes,
/// but note that Wikipedia 1-indexes them.
pub fn nth_prime(n: usize) -> usize {
    todo!()
}

/// Returns the Greatest Common Divisor (gcd) of two numbers x and y.
///
/// # Example
///
/// ```
/// use primerlab::functions::gcd;
///
/// let first_num = 60;
/// let second_num = 24;
///
/// assert_eq!(gcd(first_num, second_num), 12);
/// ```
///
/// # Note
///
/// You must use Euclid's algorithm to solve this problem. Please feel free to
/// use the World Wide Web to learn more (this applies for the entire course).
///
/// # **Restrictions**
///
/// **Please do not use the "return" keyword.**
/// **Please write this function using recursion.**
pub fn gcd(x: usize, y: usize) -> usize {
    todo!()
}

/// Returns the nth fibonacci number.
///
/// We consider the 0th fibonacci number to be 0, and the first to be 1.
///
/// # Example
///
/// ```
/// use primerlab::functions::fib;
///
/// assert_eq!(fib(2), 1);
/// assert_eq!(fib(4), 3);
/// assert_eq!(fib(7), 13);
/// ```
///
/// # Note
///
/// There are a few ways to implement this. However, one obvious way might time out on Gradescope...
///
/// The easiest way to do this is to either use iteration instead of recursion, or to use some sort
/// of helper function (which you can define somewhere else in this file).
///
/// Feel free to look this up online if you're having trouble!
pub fn fib(n: usize) -> usize {
    todo!()
}
