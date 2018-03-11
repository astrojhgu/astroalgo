extern crate astroalgo;
extern crate chrono;
extern crate num_traits;
use num_traits::float::Float;
use astroalgo::julian_day::{datetime_to_jd, jd_to_datetime};
use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono::Datelike;
use astroalgo::julian_day::ToJd;
use astroalgo::sidereal::ToMeanSidereal;
use astroalgo::precession::epoch_convert;
use astroalgo::eqpoint::EqPoint;

fn main() {
    let alpha0 = ((2.0 + 44.0 / 60.0 + 12.975 / 3600.0) / 24.0 * 360.0).to_radians();
    let delta0 = (49.0 + 13.0 / 60.0 + 39.90 / 3600.0).to_radians();
    let p0 = EqPoint {
        ra: alpha0,
        dec: delta0,
    };
    let neweq = epoch_convert(
        2451545.0,
        NaiveDate::from_ymd(2028, 11, 13).and_hms(4, 33, 36).to_jd(),
        &p0,
    );
    let alpha = neweq.ra.to_degrees();
    let delta = neweq.dec.to_degrees();
    println!("{} {}", alpha, delta);
}
