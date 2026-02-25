//! The 1 billion row challenge! Except without interacting with any I/O!

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use rowlab::{BILLION, WeatherStations, aggregate};

pub fn one_billion_row_challenge(c: &mut Criterion) {
    // Create the measurements iterator. In the real challenge, you would be reading these values
    // from a file on disk.
    let stations = WeatherStations::new();
    let measurements = stations.measurements();

    c.bench_function("brc", |b| {
        b.iter(|| {
            black_box(aggregate(measurements.clone().take(BILLION)));
        })
    });
}

criterion_main!(benches);
criterion_group! {
    name = benches;
    config = Criterion::default()
                .sample_size(10);
    targets = one_billion_row_challenge
}
