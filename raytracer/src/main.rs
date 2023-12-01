use common::*;
use sphere::Sphere;
use camera::Camera;
use hittable_list::HittableList;

use std::fs::File;
use indicatif::ProgressBar;

mod vec3;
mod ray;
mod color;
mod hittable;
mod hittable_list;
mod sphere;
mod common;
mod interval;
mod camera;

fn main() {
    let mut out = file_setup("output/book1/image7.ppm");

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(16.0 / 9.0, 400, 10);
    let mut progress = progress_bar_setup(camera.image_height());
    camera.render(&world, &mut out, &mut progress);

    std::process::exit(0);
}

// === Setup ===

fn file_setup(path_str: &str) -> File {
    let path = std::path::Path::new(path_str);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    File::create(path).unwrap()
}

fn progress_bar_setup(height: u32) -> ProgressBar {
    if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height) as u64)
    }
}
