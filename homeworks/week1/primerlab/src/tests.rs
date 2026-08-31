use crate::functions::{closest_prime, goldbach, is_prime, next_prime, nth_prime};

/// Run these tests with `cargo test`.
/// Some of these tests might be slow though. In that case, run `cargo test --release`!

#[test]
fn test_is_prime_basic() {
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(!is_prime(6));
    assert!(is_prime(7));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
    assert!(!is_prime(10));
    assert!(is_prime(11));
    assert!(!is_prime(12));
}

#[test]
fn test_is_prime_evens() {
    for i in (4..100_000_000).step_by(2) {
        assert!(!is_prime(i));
    }
}

#[test]
fn test_random_primes() {
    assert!(is_prime(2_147_483_647));
    assert!(is_prime(2_594_204_321));
    assert!(is_prime(3_499_501_457));
    assert!(!is_prime(3_875_858_981));
    assert!(!is_prime(4_123_456_789));
    assert!(is_prime(4_294_967_291));
    assert!(!is_prime(4_294_967_295));
}

#[test]
fn test_next_prime_basic() {
    assert_eq!(next_prime(2), 3);
    assert_eq!(next_prime(3), 5);
    assert_eq!(next_prime(4), 5);
    assert_eq!(next_prime(10), 11);
    assert_eq!(next_prime(37), 41);
    assert_eq!(next_prime(100), 101);
    assert_eq!(next_prime(101), 103);
}

#[test]
fn test_next_prime_larger() {
    assert_eq!(next_prime(1_000), 1_009);
    assert_eq!(next_prime(1_327), 1_361);
    assert_eq!(next_prime(5_000), 5_003);
    assert_eq!(next_prime(10_000), 10_007);
    assert_eq!(next_prime(15_683), 15_727);
    assert_eq!(next_prime(50_000), 50_021);
    assert_eq!(next_prime(98_008), 98_009);
    assert_eq!(next_prime(100_000), 100_003);
    assert_eq!(next_prime(500_000), 500_009);
    assert_eq!(next_prime(1_000_000), 1_000_003);
}

#[test]
fn test_nth_prime_basic() {
    assert_eq!(nth_prime(0), 2);
    assert_eq!(nth_prime(1), 3);
    assert_eq!(nth_prime(2), 5);
    assert_eq!(nth_prime(3), 7);
    assert_eq!(nth_prime(4), 11);
    assert_eq!(nth_prime(5), 13);
    assert_eq!(nth_prime(6), 17);
}

#[test]
fn test_nth_prime_larger() {
    assert_eq!(nth_prime(10), 31);
    assert_eq!(nth_prime(20), 73);
    assert_eq!(nth_prime(30), 127);
    assert_eq!(nth_prime(40), 179);
    assert_eq!(nth_prime(50), 233);
    assert_eq!(nth_prime(60), 283);
    assert_eq!(nth_prime(70), 353);
    assert_eq!(nth_prime(80), 419);
    assert_eq!(nth_prime(90), 467);
    assert_eq!(nth_prime(100), 547);
    assert_eq!(nth_prime(100_000), 1299721);
}

#[test]
fn test_nth_prime_very_large() {
    // This might take a long time.
    assert_eq!(nth_prime(1_000_000), 15485867);
}

#[test]
fn test_closest_prime_basic() {
    assert_eq!(closest_prime(0), 2);
    assert_eq!(closest_prime(1), 2);
    assert_eq!(closest_prime(2), 2);
    assert_eq!(closest_prime(7), 7);
    assert_eq!(closest_prime(8), 7);
    assert_eq!(closest_prime(9), 7);
    assert_eq!(closest_prime(10), 11);
    assert_eq!(closest_prime(13), 13);
    assert_eq!(closest_prime(25), 23);
    assert_eq!(closest_prime(97), 97);
}

#[test]
fn test_closest_prime_larger() {
    assert_eq!(closest_prime(1_000), 997);
    assert_eq!(closest_prime(5_000), 4_999);
    assert_eq!(closest_prime(9_569), 9_551);
    assert_eq!(closest_prime(10_000), 10_007);
    assert_eq!(closest_prime(50_000), 49_999);
    assert_eq!(closest_prime(98_008), 98_009);
    assert_eq!(closest_prime(100_000), 100_003);
    assert_eq!(closest_prime(1_000_000), 1_000_003);
}

#[test]
fn test_goldbach_basic() {
    assert_eq!(goldbach(4), 1); // 2+2
    assert_eq!(goldbach(6), 1); // 3+3
    assert_eq!(goldbach(8), 1); // 3+5
    assert_eq!(goldbach(10), 2); // 3+7, 5+5
    assert_eq!(goldbach(36), 4); // 5+31, 7+29, 13+23, 17+19
}

#[test]
fn test_goldbach_larger() {
    assert_eq!(goldbach(20), 2);
    assert_eq!(goldbach(30), 3);
    assert_eq!(goldbach(40), 3);
    assert_eq!(goldbach(50), 4);
    assert_eq!(goldbach(60), 6);
    assert_eq!(goldbach(70), 5);
    assert_eq!(goldbach(80), 4);
    assert_eq!(goldbach(90), 9);
    assert_eq!(goldbach(100), 6);
    assert_eq!(goldbach(98_008), 588);
}
