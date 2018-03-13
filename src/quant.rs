pub trait HasValue {
    fn v(&self) -> f64;
}

#[derive(Debug, Copy, Clone)]
pub struct Angle(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct Jd(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct Epoch(pub f64);

#[derive(Debug, Copy, Clone)]
pub struct Length(pub f64);

impl HasValue for Angle {
    fn v(&self) -> f64 {
        self.0
    }
}

impl HasValue for Jd {
    fn v(&self) -> f64 {
        self.0
    }
}

impl HasValue for Epoch {
    fn v(&self) -> f64 {
        self.0
    }
}

impl HasValue for Length {
    fn v(&self) -> f64 {
        self.0
    }
}
