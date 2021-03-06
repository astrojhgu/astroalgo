#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate astroalgo;
extern crate chrono;
extern crate num_traits;

use chrono::naive::NaiveDate;
use chrono::naive::NaiveDateTime;
use num_traits::float::Float;

use astroalgo::earth_position::Ecef;
use astroalgo::earth_position::LonLat;
use astroalgo::eqpoint::EqPoint;
use astroalgo::hzpoint::HzPoint;
use astroalgo::nutation;
use astroalgo::sidereal::{IntoApparentGreenSidereal, IntoMeanGreenSidereal};

use astroalgo::coord_trans;
use astroalgo::galpoint::GalPoint;
use astroalgo::parallactic;
use astroalgo::precession::epoch_convert;
use astroalgo::quant::Epoch;
use astroalgo::quant::HasValue;
use astroalgo::quant::{Angle, AsAngle};

fn main() {
    let ep = EqPoint {
        ra: Angle::from_hms(5, 19, 49.7),
        dec: Angle::from_dms(-45, 46, 44.0),
    }.at_epoch(Epoch(2000.0));
    let ep1 = ep.to_epoch(Epoch(2015.0));
    println!("{}", EqPoint::from(ep).sep(EqPoint::from(ep1)).show_dms());
}
