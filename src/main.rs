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
use astroalgo::quant::Angle;
use astroalgo::quant::HasValue;
use astroalgo::quant::Epoch;
fn main() {
    let eqpoint = EqPoint::from_radec(
        Angle::from_hms(2, 44, 12.975),
        Angle::from_dms(49, 13, 39.90),
    );

    let epoch1 = Epoch(2000.0);
    let epoch2 = Epoch::from(NaiveDate::from_ymd(2028, 11, 13).and_hms(4, 33, 36).to_jd());
    let aa: EqPoint = eqpoint.at_epoch(epoch1).to_epoch(epoch2).into();

    println!("{}", aa);
    println!(
        "{}",
        format!("{}", aa) == "2:46:11.331328730660983 49:20:54.539198835223665"
    );
}
