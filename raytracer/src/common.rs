pub use crate::color::*;
pub use crate::interval::*;
pub use crate::ray::*;
pub use crate::vec3::*;
pub use crate::common::consts::*;

mod consts {
    pub const PI: f64 = std::f64::consts::PI;
    pub const INFINITY: f64 = f64::INFINITY;
}

// === Utility Function ===

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}