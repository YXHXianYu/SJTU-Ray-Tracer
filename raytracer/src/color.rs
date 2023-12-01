use crate::{vec3::Vec3, interval::Interval};
use std::io::Write;

pub type Color = Vec3;

pub fn ppm_header(out: &mut dyn Write, width: u32, height: u32) {
    writeln!(out, "P3\n{} {}\n255", width, height).expect("Cannot write to file");
}

pub fn write_color(out: &mut dyn Write, color: Color, samples_per_pixel: u32) {

    let scale = 1.0 / samples_per_pixel as f64;
    let interval = Interval::from(0.0, 0.999);
    let trans = |x: f64| -> u8 {
        (256.0 * interval.clamp(x * scale)) as u8
    };

    let r = trans(color.x());
    let g = trans(color.y());
    let b = trans(color.z());

    writeln!(out, "{} {} {}", r, g, b).expect("Cannot write to file");
}