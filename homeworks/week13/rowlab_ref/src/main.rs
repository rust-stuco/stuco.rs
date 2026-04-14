use rowlab::{BILLION, WeatherStations, aggregate};
use std::time::Instant;

fn main() {
    // Create the measurements iterator. In the real challenge, you would be reading these values
    // from a file on disk.
    let stations = WeatherStations::new();
    let measurements = stations.measurements();

    // Record how long it takes to aggregate all 1 billion rows.
    let start = Instant::now();

    // If you want to test your code, you can make this number smaller.
    let res = aggregate(measurements.take(BILLION));

    let elapsed = start.elapsed();

    println!("Elapsed time: {:?}", elapsed);

    println!("{}", res);
}
