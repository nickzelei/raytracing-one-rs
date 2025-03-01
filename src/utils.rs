use rand::Rng;

pub const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radiants(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double() -> f64 {
    rand::random()
}

pub fn random_double_bounded(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
