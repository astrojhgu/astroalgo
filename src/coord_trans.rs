use std::convert::From;
use num_traits::float::FloatConst;

use super::earth_position::LonLat;
use super::sidereal::IntoMeanGreenSidereal;
use super::hzpoint::HzPoint;
use super::eqpoint::EqPoint;

impl EqPoint {
    pub fn hour_angle_at<'a, O, T>(&self, obs: &'a O, sd: &T) -> f64
    where
        LonLat: From<&'a O>,
        T: IntoMeanGreenSidereal,
    {
        let l = -LonLat::from(obs).lon;
        let alpha = self.ra;
        let theta0 = sd.mean_green_sidereal_angle();
        match theta0 - l - alpha {
            x if x < 0.0 => x + 2.0 * f64::PI(),
            x if x >= 2.0 * f64::PI() => x - 2.0 * f64::PI(),
            x => x,
        }
    }

    #[allow(non_snake_case)]
    pub fn hzpoint_at<'a, O, T>(&self, obs: &'a O, sd: &T) -> HzPoint
    where
        LonLat: From<&'a O>,
        T: IntoMeanGreenSidereal,
    {
        let H = self.hour_angle_at(obs, sd);
        let LonLat { lon: _, lat: phi } = LonLat::from(obs);
        let delta = self.dec;
        //let alpha=self.ra;

        let sH = H.sin();
        let cH = H.cos();
        let sphi = phi.sin();
        let cphi = phi.cos();
        let sdelta = delta.sin();
        let cdelta = delta.cos();
        let tgdelta = delta.tan();

        let A = (-sH).atan2(tgdelta * cphi - cH * sphi);
        let h = (sphi * sdelta + cphi * cdelta * cH).asin();
        HzPoint { az: A, alt: h }
    }
}

impl LonLat {
    #[allow(non_snake_case)]
    pub fn eqpoint_at<T>(&self, hzp: &HzPoint, sd: &T) -> EqPoint
    where
        T: IntoMeanGreenSidereal,
    {
        let theta0 = sd.mean_green_sidereal_angle();
        let l = -self.lon;
        let phi = self.lat;

        let A = hzp.az + f64::PI();
        let h = hzp.alt;
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
            ra: alpha,
            dec: delta,
        }
    }
}
