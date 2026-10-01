use filterlab::BloomFilter;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Taken straight from https://doc.rust-lang.org/std/hash/index.html.
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[test]
fn simple_test() {
    // https://hur.st/bloomfilter/?n=12&p=&m=128&k=1
    let mut bf = BloomFilter::new(128, 1);

    for i in (1..=12).filter(|n| n % 3 == 0) {
        bf.insert(&i);
    }

    for i in (1..=12).filter(|n| n % 3 == 0) {
        assert!(
            bf.contains(&i),
            "Bloom filters must not have false negatives"
        );
    }

    let mut false_positives = 0;
    for i in (1..12).filter(|n| n % 3 != 0) {
        if bf.contains(&i) {
            false_positives += 1;
        }
    }

    // Given the stats of the bloom filter, the false positive rate should be no more than 1 in 11.
    // If we are checking 4 elements, there shouldn't be more than 1 false positive.
    // We make it 2 elements for some wiggle room.
    assert!(
        false_positives <= 2,
        "Encountered {false_positives} false positives, should be no more than 2"
    );
}

#[test]
fn medium_test() {
    // https://hur.st/bloomfilter/?n=100&p=&m=1024&k=1
    let mut bf = BloomFilter::new(1024, 1);

    for i in (1..=300).filter(|n| n % 3 == 0) {
        bf.insert(&i);
    }

    for i in (1..=300).filter(|n| n % 3 == 0) {
        assert!(
            bf.contains(&i),
            "Bloom filters must not have false negatives"
        );
    }

    let mut false_positives = 0;
    for i in (1..300).filter(|n| n % 3 != 0) {
        if bf.contains(&i) {
            false_positives += 1;
        }
    }

    // Given the stats of the bloom filter, the false positive rate should be no more than 1 in 11.
    // If we are checking 200 elements, there shouldn't be more than 20 false positives.
    // We make it 30 elements for some wiggle room.
    assert!(
        false_positives <= 30,
        "Encountered {false_positives} false positives, should be no more than 20-30"
    );
}

#[test]
fn random_medium_test() {
    // https://hur.st/bloomfilter/?n=100&p=&m=1024&k=1
    let mut bf = BloomFilter::new(1024, 1);

    // Instead of using constants, we use an additional hash step just to make things more exciting.
    for i in (1..=300).filter(|n| n % 3 == 0) {
        let elem = calculate_hash(&i);
        bf.insert(&elem);
    }

    for i in (1..=300).filter(|n| n % 3 == 0) {
        let elem = calculate_hash(&i);
        assert!(
            bf.contains(&elem),
            "Bloom filters must not have false negatives"
        );
    }

    let mut false_positives = 0;
    for i in (1..300).filter(|n| n % 3 != 0) {
        let elem = calculate_hash(&i);
        if bf.contains(&elem) {
            false_positives += 1;
        }
    }

    // Given the stats of the bloom filter, the false positive rate should be no more than 1 in 11.
    // If we are checking 200 elements, there shouldn't be more than 20 false positives.
    // We make it 30 elements for some wiggle room.
    assert!(
        false_positives <= 30,
        "Encountered {false_positives} false positives, should be no more than 20-30"
    );
}

#[test]
fn random_large_test() {
    const MEGABYTE: usize = 1 << 20;

    // https://hur.st/bloomfilter/?n=1048576&p=&m=8388608&k=
    let mut bf = BloomFilter::new(MEGABYTE * 8, 6);

    // Instead of using constants, we use an additional hash step just to make things more exciting.
    for i in 0..MEGABYTE {
        let elem = calculate_hash(&i);
        bf.insert(&elem);
    }

    for i in 0..MEGABYTE {
        let elem = calculate_hash(&i);
        assert!(
            bf.contains(&elem),
            "Bloom filters must not have false negatives"
        );
    }

    let mut false_positives = 0;
    for i in MEGABYTE..(2 * MEGABYTE) {
        let elem = calculate_hash(&i);
        if bf.contains(&elem) {
            false_positives += 1;
        }
    }

    // Given the stats of the bloom filter, the false positive rate should be no more than 1 in 46.
    // If we are checking 1 million elements, there shouldn't be more than 23K false positives.
    assert!(
        false_positives <= 23500,
        "Encountered {false_positives} false positives, should be no more than 22K-24K"
    );
}
