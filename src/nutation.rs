//use num_traits::float::Float;

use super::quant::{Angle, Epoch};

pub struct NutCorr {
    pub dlong: Angle,
    pub dobli: Angle,
}

pub fn mean_obliquity(ep: Epoch) -> Angle {
    let jcent = (ep.0 - 2000.0) / 100.0;

    let u = jcent / 100.0;
    let ob_deg = 23.43929111111111 - 4680.93 / 3600. * u - 1.55 / 3600.0 * u.powi(2)
        + 1999.25 / 3600.0 * u.powi(3)
        - 51.38 / 3600.0 * u.powi(4)
        - 249.67 / 3600.0 * u.powi(5)
        - 39.05 / 3600.0 * u.powi(6)
        + 7.12 / 3600.0 * u.powi(7)
        + 27.87 / 3600.0 * u.powi(8)
        + 5.79 / 3600.0 * u.powi(9)
        + 2.45 / 3600.0 * u.powi(10);
    let ob = ob_deg.to_radians();
    Angle(ob)
}

#[allow(non_snake_case)]
pub fn nut_corr(ep: Epoch) -> NutCorr {
    let jc2000 = (ep.0 - 2000.0) / 100.0;
    let L = (280.4665 + 36000.7698 * jc2000).to_radians();
    let Lprime = (218.3165 + 481_267.8813 * jc2000).to_radians();

    let Omega = (125.04452 - 1934.136_261 * jc2000
        + 0.002_0708 * jc2000.powi(2)
        + jc2000.powi(3) / 450_000.0)
        .to_radians();

    let sOmega = Omega.sin();
    let cOmega = Omega.cos();
    let s2Omega = (2.0 * Omega).sin();
    let c2Omega = (2.0 * Omega).cos();

    let s2L = (2.0 * L).sin();
    let c2L = (2.0 * L).cos();
    let s2Lprime = (2.0 * Lprime).sin();
    let c2Lprime = (2.0 * Lprime).cos();

    let dpsi =
        ((-17.20 * sOmega - 1.32 * s2L - 0.23 * s2Lprime + 0.21 * s2Omega) / 3600.0).to_radians();
    let deps =
        ((9.20 * cOmega + 0.57 * c2L + 0.10 * c2Lprime - 0.09 * c2Omega) / 3600.0).to_radians();

    NutCorr {
        dlong: Angle(dpsi),
        dobli: Angle(deps),
    }
}
