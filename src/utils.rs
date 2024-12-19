use rand::Rng;

use std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn clamp(num: f64, (min, max): (f64, f64)) -> f64 {
    match num {
        _ if num < min => min,
        _ if num > max => max,
        _ => num
    }
}