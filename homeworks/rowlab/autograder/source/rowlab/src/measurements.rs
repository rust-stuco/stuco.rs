use rand::{SeedableRng, rngs::StdRng, seq::IndexedRandom};
use rand_distr::{Distribution, Normal};
use regex::Regex;

/// The file containing the different weather stations and their average temperatures.
const STATIONS_FILE: &str = "stations.txt";

/// The regex pattern to read in the possible weather stations and their average temperatures.
const STATIONS_PATTERN: &str = r#"new WeatherStation\("([^*]+)", ([^)]+)\)"#;

/// The standard deviation for the normally-distributed temperatures.
const STANDARD_DEVIATION: f64 = 20.0;

/// A very magical number. Used for seeding the random number generator.
const MAGIC_NUMBER: u64 = 42;

/// An iterator for [`WeatherStations`] that yields random measurements for random weather stations.
#[derive(Debug, Clone)]
pub struct Measurements<'a> {
    /// Weather station identifier references and normal distributions for each.
    station_distributions: Vec<(&'a str, Normal<f64>)>,
    /// A seedable random number generator.
    rng: StdRng,
}

/// The iterator implementation for an iterator over [`WeatherStations`].
impl<'a> Iterator for Measurements<'a> {
    type Item = (&'a str, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let (station, distr) = self.station_distributions.choose(&mut self.rng).unwrap();
        let measurement = distr.sample(&mut self.rng);

        Some((station, measurement))
    }
}

/// The different locations that we want to aggregate measurements for.
#[derive(Debug)]
pub struct WeatherStations {
    /// Weather station identifiers and their average temperatures.
    stations: Vec<(String, f64)>,
}

impl WeatherStations {
    pub fn new() -> Self {
        // This regex will be used to parse the weather stations list.
        let re = Regex::new(STATIONS_PATTERN).unwrap();

        // Read in the weather stations.
        let stations_content =
            std::fs::read_to_string(STATIONS_FILE).expect("unable to read stations file");

        // Parse the weather station list using regex.
        let stations: Vec<_> = stations_content
            .lines()
            .map(|line| {
                re.captures(line)
                    .map(|cap| {
                        (
                            cap.get(1).unwrap().as_str().to_string(),
                            cap.get(2).unwrap().as_str().parse().unwrap(),
                        )
                    })
                    .expect("found invalid line")
            })
            .collect::<Vec<_>>();

        Self { stations }
    }

    /// Creates a measurements iterator that yields random temperature measurements for the weather
    /// stations.
    pub fn measurements(&self) -> Measurements<'_> {
        let station_distributions: Vec<_> = self
            .stations
            .iter()
            .map(|(id, average_temperature)| {
                (
                    id.as_str(),
                    Normal::new(*average_temperature, STANDARD_DEVIATION).unwrap(),
                )
            })
            .collect();

        let rng = StdRng::seed_from_u64(MAGIC_NUMBER);

        Measurements {
            station_distributions,
            rng,
        }
    }
}

impl Default for WeatherStations {
    fn default() -> Self {
        Self::new()
    }
}
