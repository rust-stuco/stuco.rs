#![doc = include_str!("../README.md")]

use std::sync::mpsc;
use std::thread;

mod aggregation;
use aggregation::AggregationResults;

mod measurements;
pub use measurements::WeatherStations;

/// One billion.
const BILLION: usize = 1_000_000_000;

/// The number of rows each thread processes in chunks.
///
/// TODO(student): Is this a good size?
const CHUNK_SIZE: usize = 10_000;

/// Given an iterator that yields measurements for weather stations, aggregate each weather
/// station's data.
pub fn aggregate<'a, I>(mut measurements: I) -> AggregationResults
where
    I: Iterator<Item = (&'a str, f64)> + Send,
{
    assert_eq!(BILLION % CHUNK_SIZE, 0);
    let num_chunks = BILLION / CHUNK_SIZE;
    let mut results = AggregationResults::new();

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

            while let Ok(msg) = rx.try_recv() {
                results.merge_aggregation(msg);
            }
        }
    });

    // The receiver will continue to wait unless all transmitters have been dropped, so since the
    // main thread does not need one, we should drop this now.
    drop(tx);

    // Collect the aggregation results.
    while let Ok(msg) = rx.recv() {
        results.merge_aggregation(msg);
    }

    results
}
