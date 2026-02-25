use criterion::{Criterion, black_box, criterion_group, criterion_main};
use filterlab::BloomFilter;
use rand::Rng;
use rand::distr::{Bernoulli, Distribution, StandardUniform};
use rand_distr::Zipf;

/// Approximately equal to 1 million.
const MEGABYTE: usize = 1 << 20;

/// This benchmark tests the performance of `BloomFilter::contains` with a combination of both speed
/// and false positive rate. When a false positive rate occurs, we want to incur an expensive
/// operations, and in this case it is a linear search through 1 million values.
pub fn bloom_filter_read_benchmark(c: &mut Criterion) {
    let mut rng = rand::rng();

    // Generate 1 million random integers.
    let list: Vec<i32> = rng
        .clone()
        .sample_iter(StandardUniform)
        .take(MEGABYTE)
        .collect();

    // Create the bloom filter up front. We don't want to measure write speed for this benchmark.
    let mut bf = BloomFilter::new(MEGABYTE * 8, 6);
    for elem in &list {
        bf.insert(elem);
    }

    // Half of our lookups will be guaranteed to exist in the list, and the indices will be
    // distributed with a zipfian distribution so that searching for them is relatively cheap.
    let coin = Bernoulli::new(0.5).expect("0.5 is in between 0 and 1");
    let zipf = Zipf::new((MEGABYTE - 1) as f64, 1.1).unwrap();

    // We don't want to be measuring the speed of RNG, so figure out all the lookups beforehand.
    let lookups: Vec<i32> = (0..MEGABYTE)
        .map(|_| {
            if coin.sample(&mut rng) {
                // Choose a random element in the list.
                let index = rand::rng().sample(zipf) as usize;
                list[index]
            } else {
                // Lookup a random element.
                rng.random()
            }
        })
        .collect();

    let mut index: usize = 0;
    c.bench_function("read", |b| {
        b.iter(|| {
            let elem = lookups[index % lookups.len()];
            index += 1; // If only we had a way to infinitely cycle through this iterator...

            if bf.contains(black_box(&elem)) {
                // The next line could be very expensive if there was a false positive.
                let found_index = list.iter().position(|&x| black_box(x) == black_box(elem));

                // Make sure the compiler doesn't optimize this out.
                black_box(found_index);
            }
        })
    });
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = bloom_filter_read_benchmark
}
