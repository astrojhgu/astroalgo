use num_traits::float::FloatConst;
use std::convert::From;

use super::earth_position::LonLat;
use super::eqpoint::EqPoint;
use super::hzpoint::HzPoint;
use super::quant::Angle;
use super::quant::Jd;
use super::sidereal::IntoApparentGreenSidereal;
impl EqPoint {
    pub fn hour_angle_at<O, T>(&self, obs: O, sd: T) -> Angle
    where
        LonLat: From<O>,
        O: Copy,
        T: IntoApparentGreenSidereal,
        Jd: From<T>,
    {
        let l = -LonLat::from(obs).lon.0;
        let alpha = self.ra.0;
        let theta0 = sd.apparent_green_sidereal_angle().0;
        Angle(match theta0 - l - alpha {
            x if x < 0.0 => x + 2.0 * f64::PI(),
            x if x >= 2.0 * f64::PI() => x - 2.0 * f64::PI(),
            x => x,
        })
    }

    #[allow(non_snake_case)]
    pub fn hzpoint_at<O, T>(&self, obs: O, sd: T) -> HzPoint
    where
        LonLat: From<O>,
        O: Copy,
        T: IntoApparentGreenSidereal,
        Jd: From<T>,
    {
        let H = self.hour_angle_at(obs, sd).0;
        let LonLat { lon: _, lat: phi } = LonLat::from(obs);
        let delta = self.dec.0;
        //let alpha=self.ra;

        let sH = H.sin();
        let cH = H.cos();
        let sphi = phi.0.sin();
        let cphi = phi.0.cos();
        let sdelta = delta.sin();
        let cdelta = delta.cos();
        let tgdelta = delta.tan();

        let mut A = (-sH).atan2(tgdelta * cphi - cH * sphi);
        let h = (sphi * sdelta + cphi * cdelta * cH).asin();
        if A < 0.0 {
            A = A + 2.0 * f64::PI();
        }
        HzPoint {
            az: Angle(A),
            alt: Angle(h),
        }
    }
}

impl LonLat {
    #[allow(non_snake_case)]
    pub fn eqpoint_at<T>(&self, hzp: HzPoint, sd: T) -> EqPoint
    where
        T: IntoApparentGreenSidereal,
        Jd: From<T>,
    {
        //let theta0 = sd.mean_green_sidereal_angle().0;
        let theta0 = sd.apparent_green_sidereal_angle().0;
        let l = -self.lon.0;
        let phi = self.lat.0;

        let A = hzp.az.0 + f64::PI();
        let h = hzp.alt.0;
        let sA = A.sin();
        let cA = A.cos();
        let sphi = phi.sin();
        let cphi = phi.cos();

        let tgh = h.tan();
        let sh = h.sin();
        let ch = h.cos();

        let H = sA.atan2(cA * sphi + tgh * cphi);

        let delta = (sphi * sh - cphi * ch * cA).asin();

        let alpha = match theta0 - l - H {
            x if x >= 2.0 * f64::PI() => x - 2.0 * f64::PI(),
            x if x < 0.0 => x + 2.0 * f64::PI(),
            x => x,
        };

        EqPoint {
            ra: Angle(alpha),
            dec: Angle(delta),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::earth_position::LonLat;
    use super::super::quant::Angle;
    use super::super::quant::HasValue;
    use super::super::test_suit::approx;
    use chrono::naive::NaiveDate;
    use crate::hzpoint::HzPoint;
    use num_traits::float::Float;
    #[test]
    fn it_works() {
        let obs = LonLat::from_ll(Angle(120.0.to_radians()), Angle(30.0.to_radians()));
        let time = NaiveDate::from_ymd(2000, 1, 1).and_hms(12, 0, 0);
        let hz = HzPoint::from_altaz(Angle(30.0.to_radians()), Angle(45.0.to_radians()));

        let radec = obs.eqpoint_at(hz, time);
        println!("{} {}", radec.ra.v().to_degrees(), radec.dec.v().to_degrees());

        assert!(approx(radec.ra.v().to_degrees(), 118.75612284126773, 1e-10));
        assert!(approx(radec.dec.v().to_degrees(), 51.29080769669911, 1e-10));

        let hz1 = radec.hzpoint_at(obs, time);
        println!("{} {}", hz1.alt.show_dms(), hz.alt.show_dms());
        println!("{} {}", hz1.az.show_dms(), hz.az.show_dms());

        assert!(approx(hz1.alt.0, hz.alt.0, 1e-10));
        assert!(approx(hz1.az.0, hz.az.0, 1e-10));
    }
}
