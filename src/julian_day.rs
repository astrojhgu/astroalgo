use chrono::naive::{NaiveDate, NaiveDateTime};
use chrono::Datelike;
use chrono::Timelike;

pub trait ToJd {
    fn to_jd(&self) -> f64;
}

pub fn datetime_to_jd(dt: &NaiveDateTime) -> f64 {
    let (y, m) = match dt {
        &x if x.month() <= 2 => (x.year() - 1, x.month() + 12),
        &x => (x.year(), x.month()),
    };

    let d = dt.day() as f64
        + (dt.hour() as f64
            + (dt.minute() as f64 + (dt.second() as f64 + dt.nanosecond() as f64 / 1e9) / 60.0)
                / 60.0) / 24.0;

    let a = (y as f64 / 100.0).floor();
    let b = if *dt >= NaiveDate::from_ymd(1582, 10, 4).and_hms(0, 0, 0) {
        2.0 - a + (a / 4.0).floor()
    } else {
        0.0
    };
    let jd = (365.25 * (y as f64 + 4716.0)).floor() + (30.6001 * ((m + 1) as f64)).floor() + d + b
        - 1524.5;
    if jd < 0.0 {
        panic!("JD cannot < 0");
    }
    jd
}

impl ToJd for NaiveDateTime {
    fn to_jd(&self) -> f64 {
        datetime_to_jd(self)
    }
}

pub fn jd_to_datetime(jd: f64) -> NaiveDateTime {
    let (z, f) = {
        let jd1 = jd + 0.5;
        let z = jd1.trunc();
        let f = jd1 - z;
        (z, f)
    };
    let a = if z < 2299_161.0 {
        z
    } else {
        let alpha = ((z - 1867_216.25) / 36524.25).floor();
        z + 1.0 + alpha - (alpha / 4.0).floor()
    };
    let b = a + 1524.0;
    let c = ((b - 122.1) / 365.25).floor();
    let d = (365.25 * c).floor();
    let e = ((b - d) / 30.6001).floor();
    let decimal_day = b - d - (30.6001 * e).floor() + f;
    let day = decimal_day.floor() as u32;

    let fd = decimal_day - day as f64;

    println!("dec day={}", decimal_day);

    let h = (fd * 24.0).floor();
    let m = (fd * 24.0 * 60.0 - h * 60.0).floor();
    let s = (fd * 24.0 * 3600.0 - h * 3600.0 - m * 60.0).floor();
    let ns = ((fd * 24.0 * 3600.0 - h * 3600.0 - m * 60.0 - s) * 1e9) as u32;

    let month = if e >= 14.0 {
        e as u32 - 13
    } else {
        e as u32 - 1
    };
    let year = if month > 2 {
        c as i32 - 4716
    } else {
        c as i32 - 4715
    };

    NaiveDate::from_ymd(year, month, day).and_hms_nano(h as u32, m as u32, s as u32, ns)
}
