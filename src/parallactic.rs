use earth_position::LonLat;
use eqpoint::EqPoint;
use quant::Angle;
//use num_traits::float::Float;
use sidereal::IntoApparentGreenSidereal;

#[allow(non_snake_case)]
pub fn parallactic_angle(H: Angle, dec: Angle, lat: Angle) -> Angle {
    Angle(
        H.0
            .sin()
            .atan2(lat.0.tan() * dec.0.cos() - dec.0.sin() * H.0.cos()),
    )
}

impl EqPoint {
    #[allow(non_snake_case)]
    pub fn parallactic_angle_at<T, P>(&self, p: P, t: T) -> Angle
    where
        T: IntoApparentGreenSidereal,
        LonLat: From<P>,
    {
        let LonLat { lon, lat } = LonLat::from(p);
        let ra = self.ra;
        let dec = self.dec;
        let green_sdr = t.apparent_green_sidereal_angle();
        let H = Angle(green_sdr.0 - lon.0 - ra.0);
        parallactic_angle(H, dec, lat)
    }
}
