use rand::distr::{Bernoulli, Distribution};
use rand::seq::IndexedRandom;
use rand_distr::Normal;
use rowlab::aggregate;

/// Some sample weather stations.
static STATIONS: [(&str, f64); 4] = [
    ("Munich", 10.6),
    ("Amsterdam", 10.2),
    ("Pittsburgh", -42.0),
    ("Madison", 45.5),
];

/// Checks if two floating points are almost equal to each other.
fn almost_equal(a: f64, b: f64, epsilon: f64) {
    if (a - b).abs() >= epsilon {
        assert_eq!(a, b);
    }
}

#[test]
fn test_mean() {
    let measurements = (0..1000).map(|_| {
        *STATIONS
            .choose(&mut rand::rng())
            .expect("`STATIONS` is not empty")
    });

    let res = aggregate(measurements);

    for (station, true_mean) in STATIONS {
        let stats = res.get_metrics(station).expect("missing station");
        almost_equal(true_mean, stats.mean(), 0.001);
    }
}

#[test]
fn test_minmax() {
    let measurements = (0..1000).map(|_| {
        let mut rng = rand::rng();

        let (station, mean) = STATIONS.choose(&mut rng).expect("`STATIONS` is not empty");

        let coin = Bernoulli::new(0.5).expect("0.5 is between 0 and 1");
        if coin.sample(&mut rng) {
            (*station, mean + 4.2)
        } else {
            (*station, mean - 4.2)
        }
    });

    let res = aggregate(measurements);

    for (station, mean) in STATIONS {
        let stats = res.get_metrics(station).expect("missing station");

        // It's very, very unlikely that the coin flips heads 1000 times in a row.
        almost_equal(mean - 4.2, stats.min(), 0.001);
        almost_equal(mean + 4.2, stats.max(), 0.001);
    }
}

#[test]
fn test_all() {
    let measurements = (0..10_000).map(|_| {
        let mut rng = rand::rng();

        let (station, mean) = STATIONS.choose(&mut rng).expect("`STATIONS` is not empty");

        let distr = Normal::new(*mean, 1.0).unwrap();

        // Sample from the distribution but reject any measurements that are over one standard
        // deviation away from the mean.
        let measurement = loop {
            let measurement = distr.sample(&mut rng);
            if (measurement - mean).abs() < 1.0 {
                break measurement;
            }
        };

        (*station, measurement)
    });

    let res = aggregate(measurements);

    for (station, mean) in STATIONS {
        let stats = res.get_metrics(station).expect("missing station");

        almost_equal(mean, stats.mean(), 0.2);
        // It's unlikely that we don't hit something near the edges of the distribution.
        almost_equal(mean - 1.0, stats.min(), 0.1);
        almost_equal(mean + 1.0, stats.max(), 0.1);
    }
}
