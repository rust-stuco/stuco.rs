#![doc = include_str!("../README.md")]

mod aggregation;
use aggregation::AggregationResults;

mod measurements;
pub use measurements::WeatherStations;

/// One billion.
pub const BILLION: usize = 1_000_000_000;

/// Given an iterator that yields measurements for weather stations, aggregate each weather
/// station's data.
///
/// TODO(student): This is purposefully an very bad way to compute aggregations (namely, completely
/// sequentially). If you don't want to time out, you will need to introduce parallelism in some
/// manner. And even after you introduce parallelism, there are many different things you can do to
/// speed this up dramatically.
///
/// For this lab, we would encourage you to look at the reference solution after giving this a good
/// attempt on your own! Note that the reference solution is purposefully not optimized in several
/// places, and there is lots of room for improvement. We also encourage you to go online and see if
/// you can find any interesting techniques for speeding this up.
pub fn aggregate<'a, I>(measurements: I) -> AggregationResults
where
    I: Iterator<Item = (&'a str, f64)>,
{
    let mut results = AggregationResults::new();

    for (station, measurement) in measurements {
        results.insert_measurement(station, measurement);
    }

    results
}
