use std::io::Write;
use std::fs::File;

use indicatif::ProgressBar;

mod vec3;
mod ray;
mod color;
mod hittable;
mod hittable_list;
mod sphere;
mod utils;

// use vec3::Vec3;
// use vec3::Point3;
// use ray::Ray;
use utils::*;
use color::{ Color, ppm_header, write_color };
use hittable_list::HittableList;
use sphere::Sphere;

fn main() {
    let mut out = file_setup("output/book1/image5.ppm");

    let image_size = image_setup();
    let camera = camera_setup(&image_size);

    let progress = progress_bar_setup(image_size.height);

    let world = world_setup();

    ppm_header(&mut out, image_size.width, image_size.height);

    for j in 0..image_size.height {
        for i in 0..image_size.width {
            let pixel_center = camera.pixel00_loc + (i as f64 * camera.pixel_delta_u) + (j as f64 * camera.pixel_delta_v);
            let ray_direction = pixel_center - camera.camera_center;

            let ray = Ray::from(camera.camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);

            write_color(&mut out, pixel_color);
        }
        progress.inc(1);
    }
    progress.finish();

    std::process::exit(0);
}

fn ray_color(ray: &Ray, world: &HittableList) -> Color {

    if let Some(x) = world.hit(ray, 0.0, f64::INFINITY) {
        return 0.5 * (x.normal + Color::from(1.0, 1.0, 1.0));
    }

    let unit_direction = ray.direction().unit();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
}

// === Setup ===

fn world_setup() -> HittableList {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0)));

    world
}

fn file_setup(path_str: &str) -> File {
    let path = std::path::Path::new(path_str);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");
    File::create(path).unwrap()
}

fn progress_bar_setup(height: usize) -> ProgressBar {
    if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height) as u64)
    }
}

struct ImageSize {
    width: usize,
    height: usize,
}

#[allow(dead_code)]
struct Camera {
    viewport_width: f64,
    viewport_height: f64,

    focal_length: f64,

    camera_center: Vec3,
    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
}

fn image_setup() -> ImageSize {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400_usize;
    let height = std::cmp::max(1, (width as f64 / aspect_ratio) as usize);
    ImageSize { width, height }
}

fn camera_setup(image: &ImageSize) -> Camera {
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image.width as f64 / image.height as f64);
    let camera_center = Vec3::new();

    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image.width as f64;
    let pixel_delta_v = viewport_v / image.height as f64;

    let viewport_upper_left = camera_center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    Camera {
        viewport_width,
        viewport_height,
        focal_length,
        camera_center,
        viewport_u,
        viewport_v,
        pixel_delta_u,
        pixel_delta_v,
        viewport_upper_left,
        pixel00_loc
    }
}
