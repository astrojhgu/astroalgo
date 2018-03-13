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
fn main() {
    let obs = LonLat::from_ll(Angle(120.0.to_radians()), Angle(30.0.to_radians()));
    let time = NaiveDate::from_ymd(2001, 1, 1).and_hms(12, 0, 0);
    let hz = HzPoint::from_altaz(Angle(30.0.to_radians()), Angle(45.0.to_radians()));

    let radec = obs.eqpoint_at(hz, time);
    println!("{} {}", radec.ra.0.to_degrees(), radec.dec.0.to_degrees());

    let epoch1 = time.to_jd();
    let epoch2 = NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0).to_jd();

    let radec1 = epoch_convert(epoch1, epoch2, &radec);
    println!(
        "{} {}",
        radec1.ra.v().to_degrees(),
        radec1.dec.0.to_degrees()
    );
}
