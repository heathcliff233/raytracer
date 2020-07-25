use rand::Rng;
use std::f64::consts::PI;

pub fn random_double(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    min + (max - min) * rng.gen::<f64>()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}

pub fn fmin(x: f64, y: f64) -> f64 {
    if x < y {
        return x;
    }
    y
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
