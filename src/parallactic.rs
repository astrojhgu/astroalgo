use scorus::coordinates::vec2d::Vec2d;

use earth_position::LonLat;
use eqpoint::EqPoint;
use quant::Angle;
//use num_traits::float::Float;
use quant::Jd;
use sidereal::IntoApparentGreenSidereal;
#[allow(non_snake_case)]
pub fn parallactic_angle(H: Angle, dec: Angle, lat: Angle) -> Angle {
    //let (x,y)=(lat.0.tan() * dec.0.cos() - dec.0.sin() * H.0.cos(), H.0.sin());
    let Vec2d { x, y } = zenith_direction_about(H, dec, lat);

    Angle(y.atan2(x))
}

#[allow(non_snake_case)]
pub fn zenith_direction_about(H: Angle, dec: Angle, lat: Angle) -> Vec2d<f64> {
    let (x, y) = (
        lat.0.tan() * dec.0.cos() - dec.0.sin() * H.0.cos(),
        H.0.sin(),
    );
    Vec2d::new(x, y).normalized()
}

impl EqPoint {
    #[allow(non_snake_case)]
    pub fn parallactic_angle_at<T, P>(&self, p: P, t: T) -> Angle
    where
        T: IntoApparentGreenSidereal,
        Jd: From<T>,
        LonLat: From<P>,
        P: Copy,
    {
        let LonLat { lon: _lon, lat } = LonLat::from(p);
        let dec = self.dec;
        //let green_sdr = t.apparent_green_sidereal_angle();
        //let H = Angle(green_sdr.0 + lon.0 - ra.0);
        let H = self.hour_angle_at(p, t);
        parallactic_angle(H, dec, lat)
    }

    #[allow(unused_doc_comments)]
    #[allow(non_snake_case)]
    pub fn direction_of_zenith<T, P>(&self, p: P, t: T) -> Vec2d<f64>
    where
        Jd: From<T>,
        T: IntoApparentGreenSidereal,
        LonLat: From<P>,
        P: Copy,
    {
        /**
        return the direction of zenith in the framework defined in IAU convention:
        https://lambda.gsfc.nasa.gov/product/about/IAU_Conventions_v2.svg
        */
        let LonLat { lon: _lon, lat } = LonLat::from(p);

        let dec = self.dec;
        //let green_sdr = t.apparent_green_sidereal_angle();
        //let H = Angle(green_sdr.0 + lon.0 - ra.0);
        let H = self.hour_angle_at(p, t);
        //println!("H={}", H.0.to_degrees());
        zenith_direction_about(H, dec, lat)
    }
}
