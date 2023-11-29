use crate::vec3::Vec3;
use std::io::Write;

pub type Color = Vec3;

pub fn ppm_header(out: &mut dyn Write, width: usize, height: usize) {
    writeln!(out, "P3\n{} {}\n255", width, height).expect("Cannot write to file");
}

pub fn write_color(out: &mut dyn Write, color: Color) {
    let r = (255.999 * color.x()) as u8;
    let g = (255.999 * color.y()) as u8;
    let b = (255.999 * color.z()) as u8;

    writeln!(out, "{} {} {}", r, g, b).expect("Cannot write to file");
}