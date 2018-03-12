extern crate astroalgo;
extern crate chrono;
extern crate num_traits;

use chrono::naive::NaiveDate;
use num_traits::float::Float;

use astroalgo::earth_position::LonLat;
use astroalgo::eqpoint::EqPoint;
use astroalgo::hzpoint::HzPoint;

fn main() {
    let star = LonLat::from_ll(120.0.to_radians(), 30.0.to_radians()).eqpoint_at(
        &HzPoint {
            az: 120.0.to_radians(),
            alt: 50.0.to_radians(),
        },
        &NaiveDate::from_ymd(2012, 12, 1).and_hms(12, 0, 0),
    );
    println!("{} {}", star.ra.to_degrees(), star.dec.to_degrees());

    //let hz=LonLat::from(&LonLat::from_ll(12.0, 2.0));

    let hz = star.hzpoint_at(
        &LonLat::from_ll(120.0.to_radians(), 30.0.to_radians()),
        &NaiveDate::from_ymd(2012, 12, 1).and_hms(12, 0, 0),
    );
    println!("{} {}", hz.az.to_degrees(), hz.alt.to_degrees());
}
