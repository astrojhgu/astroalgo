use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::Datelike;
use crate::nutation::{mean_obliquity, nut_corr};
use crate::quant::Jd;
use crate::quant::{Angle, Epoch};

pub trait IntoMeanGreenSidereal: Sized
where
    Jd: From<Self>,
{
    fn mean_green_sidereal_angle(self) -> Angle;
}

pub trait IntoApparentGreenSidereal: IntoMeanGreenSidereal + Copy + Sized
where
    Jd: From<Self>,
{
    fn apparent_green_sidereal_angle(self) -> Angle {
        let ep = Epoch::from(Jd::from(self));
        let eps0 = mean_obliquity(ep).0;
        let nc = nut_corr(ep);
        let eps = eps0 + nc.dobli.0;

        let dphi = nc.dlong.0;
        let corr = dphi * eps.cos();
        Angle(self.mean_green_sidereal_angle().0 + corr)
    }
}

pub fn date_mean_green_sidereal_angle(date: NaiveDate) -> Angle {
    let jd = Jd::from(date.and_hms_opt(0, 0, 0).unwrap()).0;
    let t = (jd - 2_451_545.0) / 36525.0;
    let mut sdr_deg: f64 = 100.460_618_37 + 36_000.770_053_608 * t + 0.000_387_933 * t.powi(2)
        - t.powi(3) / 38_710_000.0;
    //println!("{}", t);
    sdr_deg -= (sdr_deg / 360.0).round() * 360.0;
    if sdr_deg < 0.0 {
        sdr_deg += 360.0;
    }
    Angle(sdr_deg.to_radians())
}

pub fn datetime_mean_green_sidereal_angle(datetime: &NaiveDateTime) -> Angle {
    let jd = Jd::from(*datetime).0;
    let t = (Jd::from(
        NaiveDate::from_ymd_opt(datetime.year(), datetime.month(), datetime.day()).unwrap().and_hms_opt(0, 0, 0).unwrap(),
    ).0 - 2_451_545.0)
        / 36525.0;

    let mut sdr_deg =
        280.460_618_37 + 360.985_647_366_29 * (jd - 2_451_545.0) + 0.000_387_933 * t.powi(2)
            - t.powi(3) / 38_710_000.0;
    sdr_deg -= (sdr_deg / 360.0).round() * 360.0;
    if sdr_deg < 0.0 {
        sdr_deg += 360.0;
    }
    Angle(sdr_deg.to_radians())
}

impl IntoMeanGreenSidereal for NaiveDate {
    fn mean_green_sidereal_angle(self) -> Angle {
        date_mean_green_sidereal_angle(self)
    }
}

impl IntoApparentGreenSidereal for NaiveDate {}

impl IntoMeanGreenSidereal for NaiveDateTime {
    fn mean_green_sidereal_angle(self) -> Angle {
        datetime_mean_green_sidereal_angle(&self)
    }
}

impl IntoApparentGreenSidereal for NaiveDateTime {}
