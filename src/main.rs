#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate astroalgo;
extern crate chrono;
extern crate num_traits;

use chrono::naive::NaiveDate;
use chrono::naive::NaiveDateTime;
use num_traits::float::Float;

use astroalgo::earth_position::LonLat;
use astroalgo::earth_position::Ecef;
use astroalgo::eqpoint::EqPoint;
use astroalgo::hzpoint::HzPoint;
use astroalgo::sidereal::{IntoApparentGreenSidereal, IntoMeanGreenSidereal};
use astroalgo::nutation;
use astroalgo::julian_day::ToJd;

use astroalgo::precession::epoch_convert;
use astroalgo::quant::{Angle, AsAngle};
use astroalgo::quant::HasValue;
use astroalgo::quant::Epoch;
use astroalgo::parallactic;
use astroalgo::coord_trans;
use astroalgo::galpoint::GalPoint;

fn main() {
    let ep=EqPoint{ra:Angle::from_hms(17, 48, 59.74), dec:Angle::from_dms(-14, 43, 8.2)}.at_epoch(Epoch(1950.0));

    let gal=GalPoint::from(ep);

    println!("{} {}", gal.l.0.to_degrees(),gal.b.0.to_degrees());

    let ep1=gal.to_eqpoint(Epoch(1950.0));
    //println!("{} {}", ep1.ra.show_hms(), ep1.dec.show_dms());
    println!("{} {}", ep1.ra.show_hms(), ep1.dec.show_dms());
}
