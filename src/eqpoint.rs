use std::clone::Clone;
use std::marker::Copy;

pub struct EqPoint {
    pub ra: f64,
    pub dec: f64,
}

impl Clone for EqPoint {
    fn clone(&self) -> EqPoint {
        EqPoint {
            ra: self.ra,
            dec: self.dec,
        }
    }
}

impl Copy for EqPoint {}
