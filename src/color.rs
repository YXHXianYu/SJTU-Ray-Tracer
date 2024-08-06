use crate::{vec3::Vec3, interval::Interval};
use std::io::Write;

pub type Color = Vec3;

#[allow(dead_code)]
pub fn ppm_header(out: &mut dyn Write, width: u32, height: u32) {
    writeln!(out, "P3\n{} {}\n255", width, height).expect("Cannot write to file");
}

#[inline]
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.sqrt()
}

pub fn transform_color(color: Color, samples_per_pixel: u32) -> [u8; 3] {

    let scale = 1.0 / samples_per_pixel as f64;
    let interval = Interval::from(0.0, 0.999);
    let trans = |x: f64| -> u8 {
        (256.0 * interval.clamp(linear_to_gamma(x * scale))) as u8
    };

    let r = trans(color.x());
    let g = trans(color.y());
    let b = trans(color.z());

    [r, g, b]
}
