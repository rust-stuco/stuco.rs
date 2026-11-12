#![doc = include_str!("../README.md")]

mod aggregation;
use aggregation::AggregationResults;

mod measurements;
pub use measurements::WeatherStations;

/// Given an iterator that yields measurements for weather stations, aggregate each weather
/// station's data.
pub fn aggregate<'a, I>(measurements: I) -> AggregationResults
where
    I: Iterator<Item = (&'a str, f64)>,
{
    // TODO(student): This is purposefully an very bad way to compute aggregations (namely, completely
    // sequentially). If you don't want to time out, you will need to introduce parallelism in some
    // manner. And even after you introduce parallelism, there are many different things you can do to
    // speed this up dramatically. We also encourage you to go online and see if you can find any
    // interesting optimizations!
    let mut results = AggregationResults::new();

    for (station, measurement) in measurements {
        results.insert_measurement(station, measurement);
    }

    results
}
