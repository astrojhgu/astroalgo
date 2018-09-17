extern crate chrono;
extern crate num_traits;
extern crate scorus;

#[macro_use]
extern crate serde_derive;
extern crate serde;


pub mod constants;
pub mod coord_trans;
pub mod cosmology;
pub mod earth_position;
pub mod eqpoint;
pub mod galpoint;
pub mod hzpoint;
pub mod julian_day;
pub mod nutation;
pub mod parallactic;
pub mod precession;
pub mod quant;
pub mod sidereal;
pub mod test_suit;

pub use scorus::coordinates::vec2d::Vec2d;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
