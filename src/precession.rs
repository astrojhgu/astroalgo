use super::eqpoint::EqPoint;
use num_traits::float::Float;
use num_traits::float::FloatConst;

fn sec2rad(x: f64) -> f64 {
    (x / 3600.0).to_radians()
}

pub fn epoch_convert(jd_orig: f64, jd_dest: f64, p: &EqPoint) -> EqPoint {
    let tt = (jd_orig - 2451545.0) / 36525.0;
    let t = (jd_dest - jd_orig) / 36525.0;
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

    let delta0 = p.dec;
    let alpha0 = p.ra;
    let a = delta0.cos() * (alpha0 + zeta).sin();
    let b = theta.cos() * delta0.cos() * (alpha0 + zeta).cos() - theta.sin() * delta0.sin();
    let c = theta.sin() * delta0.cos() * (alpha0 + zeta).cos() + theta.cos() * delta0.sin();

    let alpha = z + a.atan2(b);
    let delta = c.asin();
    EqPoint {
        ra: alpha,
        dec: delta,
    }
}
