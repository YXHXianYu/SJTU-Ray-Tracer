pub use crate::color::*;
pub use crate::interval::*;
pub use crate::ray::*;
pub use crate::vec3::*;
pub use crate::common::consts::*;

mod consts {
    pub const PI: f64 = std::f64::consts::PI;
    pub const INFINITY: f64 = f64::INFINITY;
    pub const EPS: f64 = 1e-9;
}

// === Utility Function ===

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * consts::PI / 180.0
}

#[allow(dead_code)]
pub fn sign(a: f64) -> isize {
    if a < -consts::EPS {
        -1
    } else if a > consts::EPS {
        1
    } else {
        0
    }
}

#[allow(dead_code)]
pub fn cmp(a: f64, b: f64) -> isize {
    sign(a - b)
}
