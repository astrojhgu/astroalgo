/*
follow the definition of
https://commons.wikimedia.org/wiki/File:Azimuth-Altitude_schematic.svg#/media/File:Azimuth-Altitude_schematic.svg
*/

use std::clone::Clone;
use std::marker::Copy;

pub struct HzPoint {
    pub az: f64,
    pub alt: f64,
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
