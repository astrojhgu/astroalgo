use std::clone::Clone;
use std::marker::Copy;
use std::convert::From;
use super::quant::{Angle, Epoch};
use super::precession::epoch_convert;
use std::fmt::{Display, Error, Formatter};

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
    pub epoch: Epoch,
}

impl Clone for EqPointAtEpoch {
    fn clone(&self) -> EqPointAtEpoch {
        EqPointAtEpoch {
            ra: self.ra,
            dec: self.dec,
            epoch: self.epoch,
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
            epoch: ep,
        }
    }

    pub fn sep(&self, another:EqPoint)->Angle{
        let d1=self.dec.0;
        let d2=another.dec.0;
        let a1=self.ra.0;
        let a2=another.ra.0;
        Angle((d1.sin()*d2.sin()+d1.cos()*d2.cos()*(a1-a2).cos()).acos())
    }
}

impl Display for EqPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.ra.show_hms(), self.dec.show_dms())
    }
}

impl EqPointAtEpoch {
    pub fn to_epoch(&self, ep: Epoch) -> EqPointAtEpoch {
        epoch_convert(
            self.epoch,
            ep,
            &EqPoint {
                ra: self.ra,
                dec: self.dec,
            },
        ).at_epoch(ep)
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
