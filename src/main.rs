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
    let eqpoint = EqPoint::from_radec(Angle::from_dms(86, 30, 0.0), Angle::from_dms(0, 0, 0.0));
    let time = NaiveDate::from_ymd(2012, 12, 1).and_hms(20, 0, 0);
    println!(
        "sidereal angle={}",
        time.apparent_green_sidereal_angle().0.to_degrees()
    );

    let pa = eqpoint.direction_of_zenith(
        LonLat::from_ll(
            40.0.to_radians().as_angle(),
            (-30.0).to_radians().as_angle(),
        ),
        time,
    );
    println!("{} {}", pa.x, pa.y);
}
