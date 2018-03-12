use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::Datelike;
use super::julian_day::ToJd;

pub trait IntoMeanGreenSidereal {
    fn mean_green_sidereal_angle(&self) -> f64;
}

pub fn date_mean_green_sidereal_angle(date: &NaiveDate) -> f64 {
    let jd = date.and_hms(0, 0, 0).to_jd();
    let t = (jd - 2451_545.0) / 36525.0;
    let mut sdr_deg: f64 = 100.460_618_37 + 36_000.770_053_608 * t + 0.000_387_933 * t.powi(2)
        - t.powi(3) / 38_710_000.0;
    println!("{}", t);
    sdr_deg -= (sdr_deg / 360.0).round() * 360.0;
    if sdr_deg < 0.0 {
        sdr_deg += 360.0;
    }
    sdr_deg.to_radians()
}

pub fn datetime_mean_green_sidereal_angle(datetime: &NaiveDateTime) -> f64 {
    let jd = datetime.to_jd();
    let t = (NaiveDate::from_ymd(datetime.year(), datetime.month(), datetime.day())
        .and_hms(0, 0, 0)
        .to_jd() - 2451_545.0) / 36525.0;

    let mut sdr_deg = 280.460_618_37 + 360.985_647_366_29 * (jd - 2451_545.0)
        + 0.000387_933 * t.powi(2) - t.powi(3) / 38_710_000.0;
    sdr_deg -= (sdr_deg / 360.0).round() * 360.0;
    if sdr_deg < 0.0 {
        sdr_deg += 360.0;
    }
    sdr_deg.to_radians()
}

impl IntoMeanGreenSidereal for NaiveDate {
    fn mean_green_sidereal_angle(&self) -> f64 {
        date_mean_green_sidereal_angle(self)
    }
}

impl IntoMeanGreenSidereal for NaiveDateTime {
    fn mean_green_sidereal_angle(&self) -> f64 {
        datetime_mean_green_sidereal_angle(self)
    }
}

impl IntoMeanGreenSidereal for f64 {
    fn mean_green_sidereal_angle(&self) -> f64 {
        *self
    }
}
