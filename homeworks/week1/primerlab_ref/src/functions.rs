/// Returns `true` if `n` is a prime number, and `false` otherwise.
///
/// # Examples
///
/// ```
/// use primerlab_ref::functions::is_prime;
///
/// assert!(is_prime(2));
/// assert!(is_prime(5));
/// assert!(!is_prime(42));
/// assert!(is_prime(113));
/// assert!(!is_prime(98_008));
/// ```
pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n.is_multiple_of(2) || n.is_multiple_of(3) {
        return false;
    }
    let max_factor = n.isqrt();
    for i in (5..=max_factor).step_by(6) {
        if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
            return false;
        }
    }
    true
}

/// Returns the next prime after `n`.
///
/// # Examples
///
/// ```
/// use primerlab_ref::functions::next_prime;
///
/// assert_eq!(next_prime(2), 3);
/// assert_eq!(next_prime(42), 43);
/// assert_eq!(next_prime(113), 127);
/// ```
pub fn next_prime(n: u32) -> u32 {
    let mut result = n + 1;
    while !is_prime(result) {
        result += 1;
    }
    result
}

/// Returns the `n`th prime, where `n` is zero-indexed.
///
/// The 0th prime is 2, the 1st prime is 3, the 2nd prime is 5, etc.
///
/// # Examples
///
/// ```
/// use primerlab_ref::functions::nth_prime;
///
/// assert_eq!(nth_prime(0), 2);
/// assert_eq!(nth_prime(4), 11);
/// assert_eq!(nth_prime(20), 73);
/// ```
pub fn nth_prime(n: u32) -> u32 {
    let mut result = 2;
    for _ in 0..n {
        result = next_prime(result);
    }
    result
}

/// Returns the closest prime to a given `n`. If both a lower and higher prime are equally close, then this returns the lower prime.
///
/// # Examples
///
/// ```
/// use primerlab_ref::functions::closest_prime;
///
/// assert_eq!(closest_prime(0), 2);
/// assert_eq!(closest_prime(7), 7);
/// assert_eq!(closest_prime(25), 23);
/// ```
pub fn closest_prime(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut low = n;
    let mut high = n;
    loop {
        if is_prime(low) {
            return low;
        }
        if is_prime(high) {
            return high;
        }
        low -= 1;
        high += 1;
    }
}

/// Returns the number of ways an even integer `n` can be expressed as the sum of two prime numbers.
///
/// The Goldbach Conjecture states that this function always returns a number >= 1 :D
///
/// # Panics
///
/// Panics if `n <= 2` or `n` is odd.
///
/// # Examples
///
/// ```
/// use primerlab_ref::functions::goldbach;
///
/// assert_eq!(goldbach(4), 1); // 2+2
/// assert_eq!(goldbach(10), 2); // 3+7, 5+5
/// assert_eq!(goldbach(36), 4); // 5+31, 7+29, 13+23, 17+19
/// ```
pub fn goldbach(n: u32) -> u32 {
    assert!(
        n > 2 && n.is_multiple_of(2),
        "n must be an even number greater than 2"
    );

    let mut count = 0;
    for i in 2..=(n / 2) {
        if is_prime(i) && is_prime(n - i) {
            count += 1;
        }
    }
    count
}
