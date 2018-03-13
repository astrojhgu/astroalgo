use num_traits::float::Float;

pub fn approx(x1: f64, x2: f64, d: f64) -> bool {
    (x2 - x1).abs() < d
}
