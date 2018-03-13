use std::clone::Clone;
use std::marker::Copy;
use super::quant::{Angle, Epoch, Jd};

pub struct EqPoint {
    pub ra: Angle,
    pub dec: Angle,
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

pub struct EqPointAtEpoch {
    pub ra: Angle,
    pub dec: Angle,
    pub jd: Jd,
}

impl Clone for EqPointAtEpoch {
    fn clone(&self) -> EqPointAtEpoch {
        EqPointAtEpoch {
            ra: self.ra,
            dec: self.dec,
            jd: self.jd,
        }
    }
}

impl Copy for EqPointAtEpoch {}

impl EqPoint {
    pub fn from_radec(ra: Angle, dec: Angle) -> EqPoint {
        EqPoint { ra: ra, dec: dec }
    }

    pub fn at_epoch(&self, ep: Epoch) -> EqPointAtEpoch {
        EqPointAtEpoch {
            ra: self.ra,
            dec: self.dec,
            jd: Jd::from(ep),
        }
    }
}
