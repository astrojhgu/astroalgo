use num_traits::float::Float;
use std::convert::From;
use std::fmt::{Display, Error, Formatter};

use crate::eqpoint::EqPointAtEpoch;
use crate::quant::{Angle, Epoch};
//use std::fmt::{Formatter};

#[derive(Copy, Clone)]
pub struct GalPoint {
    pub l: Angle,
    pub b: Angle,
}

impl Display for GalPoint {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{} {}", self.l.show_hms(), self.b.show_dms())
    }
}

impl From<EqPointAtEpoch> for GalPoint {
    fn from(eqep: EqPointAtEpoch) -> GalPoint {
        let eq1950 = eqep.to_epoch(Epoch(1950.0));
        let alpha = eq1950.ra.0;
        let delta = eq1950.dec.0;

        let d1 = 3.355_395_486_959_098_5; //192.25 deg
        let d2 = 0.478_220_215_046_446_3; //27.4 deg
        let d3 = 5.288_347_633_542_818; //303 deg

        let x = f64::atan2(
            (d1 - alpha).sin(),
            (d1 - alpha).cos() * d2.sin() - delta.tan() * d2.cos(),
        );
        let l = d3 - x;
        let b = (delta.sin() * d2.sin() + delta.cos() * d2.cos() * (d1 - alpha).cos()).asin();
        GalPoint {
            l: Angle(l),
            b: Angle(b),
        }
    }
}

impl GalPoint {
    pub fn to_eqpoint(&self, ep: Epoch) -> EqPointAtEpoch {
        let d1 = 2.146_754_979_953_025_4; //123 deg
        let d2 = 0.478_220_215_046_446_3; //27.4 deg
        let d3 = 0.213_802_833_369_305_36; //12.25 deg
        let l = self.l.0;
        let b = self.b.0;
        let y = (l - d1)
            .sin()
            .atan2((l - d1).cos() * d2.sin() - b.tan() * d2.cos());
        //println!("y={}", y);
        let alpha = y + d3;
        let delta = (b.sin() * d2.sin() + b.cos() * d2.cos() * (l - d1).cos()).asin();
        EqPointAtEpoch {
            ra: Angle(alpha),
            dec: Angle(delta),
            epoch: Epoch(1950.0),
        }.to_epoch(ep)
    }
}
