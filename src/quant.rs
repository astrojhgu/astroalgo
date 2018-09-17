use super::julian_day::datetime_to_jd;
use chrono::naive::{NaiveDate, NaiveDateTime};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Angle(pub f64);

pub trait AsAngle {
    fn as_angle(&self) -> Angle;
}

impl AsAngle for f64 {
    fn as_angle(&self) -> Angle {
        Angle(*self)
    }
}

impl Angle {
    pub fn from_deg(d: f64) -> Angle {
        Angle(d.to_radians())
    }

    pub fn from_dms(d: i32, m: u32, s: f64) -> Angle {
        let df = d as f64;
        let mf = m as f64;
        let sf = s as f64;

        Angle((df.signum() * (df.abs() + mf / 60.0 + sf / 3600.0)).to_radians())
    }

    pub fn from_hms(h: u32, m: u32, s: f64) -> Angle {
        Angle(((h as f64 + m as f64 / 60.0 + s as f64 / 3600.0) * 15.0).to_radians())
    }

    pub fn show_dms(&self) -> String {
        let deg = self.0.to_degrees();
        let sign = deg.signum();
        let deg = deg.abs();
        let d = deg.floor() as u32;
        let m = ((deg - d as f64) * 60.0).floor() as u32;
        let s = (deg - d as f64 - m as f64 / 60.0) * 3600.0;
        format!(
            "{}{}:{}:{}",
            if sign > 0.0 { '+' } else { '-' },
            d as i32,
            m,
            s
        )
    }

    pub fn show_hms(&self) -> String {
        //let deg = self.0.to_degrees() - (self.0.to_degrees() / 360.0).floor() * 360.0;

        let mut ha = self.0.to_degrees() / 15.0;

        //let ha = ha.abs();
        while ha < 0.0 {
            ha += 24.0;
        }
        while ha >= 24.0 {
            ha -= 24.0;
        }
        let h = ha.floor() as u32;

        let m = ((ha - h as f64) * 60.0).floor() as u32;
        let s = (ha - h as f64 - m as f64 / 60.0) * 3600.0;
        format!("{}:{}:{}", h, m, s)
    }
}

//////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct Jd(pub f64);

impl From<NaiveDateTime> for Jd {
    fn from(ndt: NaiveDateTime) -> Jd {
        datetime_to_jd(&ndt)
    }
}

impl From<NaiveDate> for Jd {
    fn from(nd: NaiveDate) -> Jd {
        datetime_to_jd(&(nd.and_hms(0, 0, 0)))
    }
}

/////////////////////////////////
#[derive(Debug, Copy, Clone)]
pub struct Epoch(pub f64);

impl From<Jd> for Epoch {
    fn from(jd: Jd) -> Epoch {
        Epoch(2000.0 + (jd.0 - 2451_545.0) / 365.25)
    }
}

impl From<Epoch> for Jd {
    fn from(ep: Epoch) -> Jd {
        Jd((ep.0 - 2000.0) * 365.25 + 2451_545.0)
    }
}

impl From<NaiveDateTime> for Epoch {
    fn from(ndt: NaiveDateTime) -> Epoch {
        Epoch::from(Jd::from(ndt))
    }
}

/////////////////////////////////
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Length(pub f64);

pub trait AsLength {
    fn as_length(&self) -> Length;
}

impl AsLength for f64 {
    fn as_length(&self) -> Length {
        Length(*self)
    }
}

/////////////////////////////////
impl HasValue for Angle {
    fn v(&self) -> f64 {
        self.0
    }
}

//////////////
pub trait HasValue {
    fn v(&self) -> f64;
}

impl HasValue for Jd {
    fn v(&self) -> f64 {
        self.0
    }
}

impl HasValue for Epoch {
    fn v(&self) -> f64 {
        self.0
    }
}

impl HasValue for Length {
    fn v(&self) -> f64 {
        self.0
    }
}
