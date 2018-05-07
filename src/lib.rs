extern crate chrono;
extern crate num_traits;
extern crate scorus;

pub mod julian_day;
pub mod sidereal;
pub mod precession;
pub mod eqpoint;
pub mod galpoint;
pub mod earth_position;
pub mod coord_trans;
pub mod hzpoint;
pub mod nutation;
pub mod quant;
pub mod parallactic;
pub mod cosmology;
pub mod constants;
pub mod test_suit;

pub use scorus::coordinates::vec2d::Vec2d;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
