/*
follow the definition of
https://commons.wikimedia.org/wiki/File:Azimuth-Altitude_schematic.svg#/media/File:Azimuth-Altitude_schematic.svg
*/

use std::clone::Clone;
use std::marker::Copy;

use crate::quant::Angle;

#[derive(Serialize, Deserialize, Debug)]
pub struct HzPoint {
    pub az: Angle,
    pub alt: Angle,
}

impl Clone for HzPoint {
    fn clone(&self) -> HzPoint {
        HzPoint {
            az: self.az,
            alt: self.alt,
        }
    }
}

impl Copy for HzPoint {}

impl HzPoint {
    pub fn from_altaz(alt: Angle, az: Angle) -> HzPoint {
        HzPoint { alt: alt, az: az }
    }
}
