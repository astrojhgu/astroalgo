use crate::precession::epoch_convert;
use crate::quant::{Angle, Epoch};
use std::convert::From;
use std::fmt::{Display, Error, Formatter};
use std::marker::Copy;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct EqPoint {
    pub ra: Angle,
    pub dec: Angle,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct EqPointAtEpoch {
    pub ra: Angle,
    pub dec: Angle,
    pub epoch: Epoch,
}

impl EqPoint {
    pub fn from_radec(ra: Angle, dec: Angle) -> EqPoint {
        EqPoint { ra: ra, dec: dec }
    }

    pub fn at_epoch(&self, ep: Epoch) -> EqPointAtEpoch {
        EqPointAtEpoch {
            ra: self.ra,
            dec: self.dec,
            epoch: ep,
        }
    }

    pub fn sep(&self, another: EqPoint) -> Angle {
        let d1 = self.dec.0;
        let d2 = another.dec.0;
        let a1 = self.ra.0;
        let a2 = another.ra.0;
        Angle((d1.sin() * d2.sin() + d1.cos() * d2.cos() * (a1 - a2).cos()).acos())
    }
}

impl Display for EqPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.ra.show_hms(), self.dec.show_dms())
    }
}

impl EqPointAtEpoch {
    pub fn to_epoch<T>(&self, ep: T) -> EqPointAtEpoch
    where
        Epoch: From<T>,
        T: Copy,
    {
        epoch_convert(
            self.epoch,
            Epoch::from(ep),
            &EqPoint {
                ra: self.ra,
                dec: self.dec,
            },
        ).at_epoch(Epoch::from(ep))
    }
}

impl From<EqPointAtEpoch> for EqPoint {
    fn from(eqep: EqPointAtEpoch) -> EqPoint {
        EqPoint {
            ra: eqep.ra,
            dec: eqep.dec,
        }
    }
}
