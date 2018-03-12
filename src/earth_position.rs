use std::clone::Clone;
use std::marker::Copy;
use std::convert::From;

const SEMI_MAJOR_AXIS: f64 = 6_378_137.0;
const SEMI_MINOR_AXIS: f64 = 6_356_752.3;

pub struct LonLat {
    pub lon: f64,
    pub lat: f64,
}

impl Clone for LonLat {
    fn clone(&self) -> Self {
        LonLat {
            lon: self.lon,
            lat: self.lat,
        }
    }
}

impl Copy for LonLat {}

pub struct LonLatHeight {
    pub lon: f64,
    pub lat: f64,
    pub height: f64,
}

impl Clone for LonLatHeight {
    fn clone(&self) -> Self {
        LonLatHeight {
            lon: self.lon,
            lat: self.lat,
            height: self.height,
        }
    }
}

impl Copy for LonLatHeight {}

impl LonLat {
    pub fn from_ll(lon: f64, lat: f64) -> LonLat {
        LonLat { lon: lon, lat: lat }
    }

    pub fn with_height(&self, height: f64) -> LonLatHeight {
        LonLatHeight {
            lon: self.lon,
            lat: self.lat,
            height: height,
        }
    }
}

pub struct Ecef {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Ecef {
    pub fn from_xyz(x: f64, y: f64, z: f64) -> Ecef {
        Ecef { x: x, y: y, z: z }
    }
}

fn normal(lat: f64) -> f64 {
    let a = SEMI_MAJOR_AXIS;
    let b = SEMI_MINOR_AXIS;
    let e2 = 1.0 - (b / a).powi(2);
    a / (1.0 - e2 * lat.sin().powi(2)).sqrt()
}

pub fn llh2xyz(llh: &LonLatHeight) -> Ecef {
    //https://en.wikipedia.org/wiki/Geographic_coordinate_conversion#From_geodetic_to_ECEF_coordinates

    let a = SEMI_MAJOR_AXIS;
    let b = SEMI_MINOR_AXIS;

    let phi = llh.lat;
    let lambda = llh.lon;
    let h = llh.height;
    let n = normal(phi);
    let cphi = phi.cos();
    let sphi = phi.sin();
    let clambda = lambda.cos();
    let slambda = lambda.sin();
    let x = (n + h) * cphi * clambda;
    let y = (n + h) * cphi * slambda;
    let z = ((b / a).powi(2) * n + h) * sphi;
    Ecef::from_xyz(x, y, z)
}

#[allow(non_snake_case)]
pub fn xyz2llh(xyz: &Ecef) -> LonLatHeight {
    //https://en.wikipedia.org/wiki/Geographic_coordinate_conversion#From_ECEF_to_geodetic_coordinates
    let a = SEMI_MAJOR_AXIS;
    let b = SEMI_MINOR_AXIS;

    let x = xyz.x;
    let y = xyz.y;
    let z = xyz.z;

    let r = (x.powi(2) + y.powi(2)).sqrt();
    let e_prime_square = (a.powi(2) - b.powi(2)) / b.powi(2);
    let e_square = 1.0 - (b / a).powi(2);
    let E_square = a.powi(2) - b.powi(2);
    let F = 54.0 * b.powi(2) * z.powi(2);
    let G = r.powi(2) + (1.0 - e_square) * z.powi(2) - e_square * E_square;
    let C = e_square.powi(2) * F * r.powi(2) / G.powi(3);
    let S = (1.0 + C + (C.powi(2) + 2.0 * C).sqrt()).cbrt();
    let P = F / (3.0 * (S + 1.0 / S + 1.0).powi(2) * G.powi(2));
    let Q = (1.0 + 2.0 * e_square.powi(2) * P).sqrt();
    let r0 = -P * e_square * r / (1.0 + Q)
        + (0.5 * a * a * (1.0 + 1.0 / Q) - P * (1.0 - e_square) * z * z / (Q * (1.0 + Q))
            - 0.5 * P * r.powi(2))
            .sqrt();
    let U = ((r - e_square * r0).powi(2) + z.powi(2)).sqrt();
    let V = ((r - e_square * r0).powi(2) + (1.0 - e_square) * z.powi(2)).sqrt();
    let Z0 = b.powi(2) * z / (a * V);
    let h = U * (1.0 - b.powi(2) / (a * V));
    let phi = (z + e_prime_square * Z0).atan2(r);
    let lambda = y.atan2(x);
    LonLatHeight {
        lon: lambda,
        lat: phi,
        height: h,
    }
}

impl<'a> From<&'a LonLatHeight> for Ecef {
    fn from(llh: &LonLatHeight) -> Ecef {
        llh2xyz(llh)
    }
}

impl From<LonLatHeight> for Ecef {
    fn from(llh: LonLatHeight) -> Ecef {
        llh2xyz(&llh)
    }
}

impl<'a> From<&'a Ecef> for LonLatHeight {
    fn from(xyz: &Ecef) -> LonLatHeight {
        xyz2llh(xyz)
    }
}

impl From<Ecef> for LonLatHeight {
    fn from(xyz: Ecef) -> LonLatHeight {
        xyz2llh(&xyz)
    }
}

impl<'a> From<&'a LonLatHeight> for LonLat {
    fn from(llh: &LonLatHeight) -> LonLat {
        LonLat {
            lon: llh.lon,
            lat: llh.lat,
        }
    }
}

impl From<LonLatHeight> for LonLat {
    fn from(llh: LonLatHeight) -> LonLat {
        LonLat {
            lon: llh.lon,
            lat: llh.lat,
        }
    }
}

impl<'a> From<&'a LonLat> for LonLat {
    fn from(ll: &'a LonLat) -> LonLat {
        LonLat {
            lon: ll.lon,
            lat: ll.lat,
        }
    }
}
