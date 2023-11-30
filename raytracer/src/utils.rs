pub use crate::vec3::*;
pub use crate::ray::*;
pub use crate::utils::consts::*;

mod consts {
    pub const PI: f64 = std::f64::consts::PI;
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
