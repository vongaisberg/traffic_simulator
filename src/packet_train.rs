use std::time::{Duration, Instant};

use rand::prelude::*;
use rand_distr::{Distribution, Normal, Weibull};

pub struct PacketTrain {
    pub off_dist: Weibull<f64>,
    off_mean: f64,
    pub on_dist: Normal<f64>,
    pub size_dist: Normal<f64>,
    pub bandwith_dist: Normal<f64>,

    pub currently_on: bool,
    pub current_status_since: Instant,
    pub current_status_until: Instant,
    pub current_bandwith: f64,

    pub sum_of_data: f64,
}

impl PacketTrain {
    pub fn new(
        off_dist_scale: f64,
        off_dist_shape: f64,
        on_dist: Normal<f64>,
        size_dist: Normal<f64>,
        bandwith_dist: Normal<f64>,
    ) -> PacketTrain {
        PacketTrain {
            off_dist: Weibull::new(off_dist_scale, off_dist_shape).unwrap(),
            off_mean: calculate_weibull_mean(off_dist_scale, off_dist_shape),
            on_dist,
            size_dist,
            bandwith_dist,
            currently_on: false,
            current_status_since: Instant::now(),
            current_status_until: Instant::now(),
            current_bandwith: 0.,
            sum_of_data: 0.,
        }
    }

    pub fn flip_status(&mut self) {
        self.currently_on = !self.currently_on;
        if self.currently_on {
            self.current_status_since = Instant::now();
            self.current_status_until = Instant::now()
                .checked_add(Duration::from_micros(
                    (self.on_dist.sample(&mut rand::thread_rng()) * 1000.) as u64,
                ))
                .unwrap();
            self.current_bandwith = self.bandwith_dist.sample(&mut rand::thread_rng());
            self.sum_of_data = 0.;
        } else {
            self.current_status_until = Instant::now()
                .checked_add(Duration::from_micros(
                    (self.off_dist.sample(&mut rand::thread_rng()) * 1000.) as u64,
                ))
                .unwrap();
            self.current_bandwith = 0.;
        }
    }

    pub fn mean_bandwith(&self) -> f64 {
        let mean_on = self.on_dist.mean();
        let mean_off = self.off_mean;
        let fraction_on = mean_on / (mean_on + mean_off);
        println!(
            "Mean off-time: {:.3}s, Fraction on: {:.3}, Mean bandwidth: {:.3}Mbit/s",
            mean_off/1000.,
            fraction_on,
            fraction_on * self.bandwith_dist.mean() / 1_000_000. * 8.
        );
        fraction_on * self.bandwith_dist.mean()
    }
}

pub fn calculate_weibull_mean(scale: f64, shape: f64) -> f64 {
    (scale) * ((1. / shape) + 1.).gamma()
}
