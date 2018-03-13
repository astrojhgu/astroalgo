use super::eqpoint::EqPoint;
use super::quant::{Angle, Epoch, Jd};

fn sec2rad(x: f64) -> f64 {
    (x / 3600.0).to_radians()
}

pub fn epoch_convert(ep0: Epoch, ep: Epoch, p: &EqPoint) -> EqPoint {
    let tt = (ep0.0 - 2000.0) / 100.0;
    let t = (ep.0 - ep0.0) / 100.0;
    //let tt = (jd_orig.0 - 2451545.0) / 36525.0;
    //let t = (jd_dest.0 - jd_orig.0) / 36525.0;
    let zeta = sec2rad(
        (2306.2181 + 1.39656 * tt - 0.000139 * tt.powi(2)) * t
            + (0.30188 - 0.000344 * tt) * t.powi(2) + 0.017998 * t.powi(3),
    );
    let z = sec2rad(
        (2306.2181 + 1.39656 * tt - 0.000139 * tt.powi(2)) * t
            + (1.09468 + 0.000066 * tt) * t.powi(2) + 0.018203 * t.powi(3),
    );
    let theta = sec2rad(
        (2004.3109 - 0.85330 * tt - 0.000217 * tt.powi(2)) * t
            - (0.42665 + 0.000217 * tt) * t.powi(2) - 0.041833 * t.powi(3),
    );

    let delta0 = p.dec.0;
    let alpha0 = p.ra.0;
    let a = delta0.cos() * (alpha0 + zeta).sin();
    let b = theta.cos() * delta0.cos() * (alpha0 + zeta).cos() - theta.sin() * delta0.sin();
    let c = theta.sin() * delta0.cos() * (alpha0 + zeta).cos() + theta.cos() * delta0.sin();

    let alpha = z + a.atan2(b);
    let delta = c.asin();
    EqPoint {
        ra: Angle(alpha),
        dec: Angle(delta),
    }
}

#[cfg(test)]
mod tests {
    use super::super::eqpoint::EqPoint;
    use super::super::quant::Angle;
    use super::super::quant::Epoch;
    use chrono::naive::NaiveDate;
    use julian_day::ToJd;
    #[test]
    fn it_works() {
        let eqpoint = EqPoint::from_radec(
            Angle::from_hms(2, 44, 12.975),
            Angle::from_dms(49, 13, 39.90),
        );

        let epoch1 = Epoch(2000.0);
        let epoch2 = Epoch::from(NaiveDate::from_ymd(2028, 11, 13).and_hms(4, 33, 36).to_jd());
        let aa: EqPoint = eqpoint.at_epoch(epoch1).to_epoch(epoch2).into();

        assert!(format!("{}", aa) == "2:46:11.331328730660983 49:20:54.539198835223665");
    }
}
