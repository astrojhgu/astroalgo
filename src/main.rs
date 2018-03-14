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

fn main() {
    let eqpoint = EqPoint::from_radec(
        Angle::from_hms(2, 44, 12.975),
        Angle::from_dms(49, 13, 39.90),
    );

    let pa = eqpoint.parallactic_angle_at(
        LonLat::from_ll(0.0.to_radians().as_angle(), (-90.0).to_radians().as_angle()),
        NaiveDate::from_ymd(2012, 12, 1).and_hms(12, 0, 0),
    );
    println!("{}", pa.show_dms());

    let obs = LonLat::from_ll(120.0.to_radians().as_angle(), 30.0.to_radians().as_angle());
    obs.eqpoint_at(
        HzPoint::from_altaz(30.0.to_radians().as_angle(), 45.0.to_radians().as_angle()),
        NaiveDate::from_ymd(2012, 1, 1).and_hms(12, 0, 0),
    );
    println!(
        "{}",
        parallactic::parallactic_angle(
            120.0.to_radians().as_angle(),
            60.0.to_radians().as_angle(),
            30.0.to_radians().as_angle()
        ).show_dms()
    );
}
