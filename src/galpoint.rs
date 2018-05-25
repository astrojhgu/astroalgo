use num_traits::float::Float;
use std::convert::From;
use std::fmt::{Display, Error, Formatter};

use super::eqpoint::EqPointAtEpoch;
use super::quant::{Angle, Epoch};
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

        let d1 = 3.3553954869590985; //192.25 deg
        let d2 = 0.4782202150464463; //27.4 deg
        let d3 = 5.288347633542818; //303 deg

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
        let d1 = 2.1467549799530254; //123 deg
        let d2 = 0.4782202150464463; //27.4 deg
        let d3 = 0.21380283336930536; //12.25 deg
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
