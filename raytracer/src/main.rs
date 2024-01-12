use std::fs::File;
use std::rc::Rc;
use indicatif::ProgressBar;

use common::*;
use sphere::Sphere;
use camera::Camera;
use hittable_list::HittableList;
use material::{ Lambertian, Metal };

mod vec3;
mod ray;
mod color;
mod hittable;
mod hittable_list;
mod sphere;
mod common;
mod interval;
mod camera;
mod material;

fn main() {
    let mut out = file_setup("output/book1/image15.ppm");

    // material
    let material_ground = Rc::new(Lambertian::from(&Color::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::from(&Color::from(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::from(&Color::from(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::from(&Color::from(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0),    0.5,   material_center)));
    world.add(Box::new(Sphere::from(Point3::from(-1.0, 0.0, -1.0),   0.5,   material_left)));
    world.add(Box::new(Sphere::from(Point3::from(1.0, 0.0, -1.0),    0.5,   material_right)));

    let camera = Camera::new(16.0 / 9.0, 720, 20, 50);
    // let camera = Camera::new(16.0 / 9.0, 400, 10, 10);
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
