#![doc = include_str!("../README.md")]

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::mpsc;
use std::thread;

mod aggregation;
use aggregation::AggregationResults;

mod measurements;
pub use measurements::WeatherStations;

/// One billion.
pub const BILLION: usize = 1_000_000_000;

/// The number of rows each thread processes in chunks.
///
/// TODO(student): Is this a good size?
const CHUNK_SIZE: usize = 10_000;

/// Given an iterator that yields measurements for weather stations, aggregate each weather
/// station's data.
///
/// TODO(student): This is purposefully an very bad way to compute aggregations (namely, completely
/// sequentially). If you don't want to time out, you will need to introduce parallelism in some
/// manner. And even after you introduce parallelism, there are many different things you can do to
/// speed this up dramatically.
///
/// Also note that you are likely going to be bottlenecked by the input iterator. This is expected.
/// In the real 1 billion row challenge, the measurements came from a file, which would possibly be
/// even slower in some scenarios. Of course, if you make use of specific linux OS syscalls
/// (specifically `mmap`), you could eliminate a large amount of overhead. Regardless, for this
/// assignment the lower bound is approximately the same as the time it takes to run this function
/// but with the `s.spawn` completely commented out.
pub fn aggregate<'a, I>(mut measurements: I) -> AggregationResults
where
    I: Iterator<Item = (&'a str, f64)> + Send,
{
    assert_eq!(BILLION % CHUNK_SIZE, 0);
    let num_chunks = BILLION / CHUNK_SIZE;
    let mut chunk_results = Vec::with_capacity(num_chunks);

    // Create an `mpsc` channel that threads will use to send their aggregation results back to the
    // main (current) thread.
    // Note that you can achieve the exact same effect via a mutex-protected vector. Try it out!
    let (tx, rx) = mpsc::channel();

    // This scope is just an more ergonomic way to spawn threads and wait for all of them to finish
    // (join all threads), while also allowing the threads to access local data (like the
    // measurements iterator and the channel, for example).
    thread::scope(|s| {
        for _ in 0..num_chunks {
            let chunk = measurements.by_ref().take(CHUNK_SIZE).collect::<Vec<_>>();

            // Spawn a thread to process each chunk.
            s.spawn(|| {
                let mut results = AggregationResults::new();

                for (station, measurement) in chunk {
                    results.insert_measurement(station, measurement);
                }

                tx.send(results)
            });
        }
    });

    // The receiver will continue to wait unless all transmitters have been dropped, so since the
    // main thread does not need one, we should drop this now.
    drop(tx);

    // Collect the aggregation results.
    while let Ok(msg) = rx.recv() {
        chunk_results.push(msg);
    }

    // Once we have the results, use `rayon`'s `ParallelIterator` to combine all of them (reduce)
    // into a single value.
    // Note that you really don't need to (and probably shouldn't) do this. It's likely that the
    // single-threaded version is faster, depending on how many chunks you have.
    // This is just demonstrating the `rayon` library and how easy it is to parallelize things in
    // Rust :D.
    chunk_results
        .into_par_iter()
        .reduce(AggregationResults::new, |mut a, b| {
            a.merge_aggregation(b);
            a
        })
}
