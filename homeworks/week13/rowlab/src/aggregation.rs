use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Write};

/// Aggregate statistics for a specific [`WeatherStation`].
#[derive(Debug, Clone, Copy)]
pub struct StationAggregation {
    /// The minimum temperature measurement.
    min: f64,
    /// The maximum temperature measurement.
    max: f64,
    /// The average / mean temperature measurement.
    mean: f64,
    /// Helper field for calculating mean (sum_measurements / num_measurements).
    sum_measurements: f64,
    /// Helper field for calculating mean (sum_measurements / num_measurements).
    num_measurements: f64,
}

impl StationAggregation {
    /// Creates a new `StationAggregation` for computing aggregations.
    pub fn new() -> Self {
        Self {
            min: f64::INFINITY,
            mean: 0.0,
            max: f64::NEG_INFINITY,
            sum_measurements: 0.0,
            num_measurements: 0.0,
        }
    }

    /// Updates the aggregation with a new measurement.
    ///
    /// TODO(student): Is processing measurements one-by-one the best way to compute aggregations?
    /// Remember that you are allowed to add other methods in this implementation block!
    pub fn add_measurement(&mut self, measurement: f64) {
        todo!("Implement me!")
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }
}

impl Display for StationAggregation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.1}/{:.1}/{:.1}", self.min, self.mean, self.max)
    }
}

/// The aggregation results for the billion row challenge.
///
/// TODO(student): This is purposefully not an ideal structure! You are allowed to change what
/// types this struct contains. Think about what this structure should represent, and where the data
/// might best be located. Also, you are allowed to use third-party data structures.
#[derive(Debug)]
pub struct AggregationResults {
    /// A map from weather station identifier to its aggregate metrics.
    results: HashMap<String, StationAggregation>,
}

impl AggregationResults {
    /// Creates an empty `AggregationResult`.
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    /// Updates the metrics for the given station with a measurement.
    pub fn insert_measurement(&mut self, station: &str, measurement: f64) {
        todo!("Implement me!")
    }

    /// Retrieve the stats of a specific station, if it exists. Used for testing purposes.
    pub fn get_metrics(&self, station: &str) -> Option<StationAggregation> {
        self.results.get(station).copied()
    }
}

impl Display for AggregationResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Sort the results by weather station ID and join into the output string format.
        let sorted_results: Vec<_> = self
            .results
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .collect();

        f.write_char('{')?;

        // Append each weather station's metrics to the output string.
        for (station, aggregation) in sorted_results.iter().take(sorted_results.len() - 1) {
            f.write_str(station)?;
            f.write_char('=')?;
            // Note that implementing `Display` on `StationAggregation` means that you can call
            // `to_string` and it will do a similar thing as `Display::fmt`.
            f.write_str(&aggregation.to_string())?;
            f.write_char(',')?;
            f.write_char(' ')?;
        }

        let (last_station, last_aggregation) =
            sorted_results.last().expect("somehow empty results");
        f.write_str(last_station)?;
        f.write_char('=')?;
        f.write_str(&last_aggregation.to_string())?;

        f.write_char('}')
    }
}

impl Default for StationAggregation {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for AggregationResults {
    fn default() -> Self {
        Self::new()
    }
}
