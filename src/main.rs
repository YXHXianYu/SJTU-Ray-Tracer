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

use std::rc::Rc;
use std::env;
use std::fs::File;

use indicatif::ProgressBar;
use console::style;
use image::{ ImageBuffer, RgbImage };

use common::*;
use sphere::Sphere;
use camera::Camera;
use camera::CameraCreateInfo;
use hittable_list::HittableList;
use material::{ Lambertian, Metal, Dielectric };

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let path = if args.len() == 1 {
        "output/book1/tmp.jpg"
    } else if args.len() == 2 {
        &args[1]
    } else {
        panic!("Image output path error.")
    };
    let mut output_file = file_setup(path);

    let camera = Camera::new(CameraCreateInfo{
        samples_per_pixel: 10,
        max_depth: 10,
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        vfov: 90.0,
    });
    let mut progress = progress_bar_setup(camera.image_height());

    let mut img: RgbImage = ImageBuffer::new(camera.image_width(), camera.image_height());

    let world = get_world2();

    camera.render(&world, &mut img, &mut progress);

    println!(
        "Ouput image as \"{}\"",
        style(path).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(100)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

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

#[allow(dead_code)]
fn get_world1() -> HittableList {
    let material_ground = Rc::new(Lambertian::from(&Color::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::from(&Color::from(0.1, 0.2, 0.5)));
    let material_left   = Rc::new(Dielectric::from(1.5));
    let material_right  = Rc::new(Metal::from(&Color::from(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::from(Point3::from(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0),    0.5,   material_center)));
    world.add(Box::new(Sphere::from(Point3::from(-1.0, 0.0, -1.0),   0.5,   material_left.clone())));
    world.add(Box::new(Sphere::from(Point3::from(-1.0, 0.0, -1.0),   -0.4,  material_left)));
    world.add(Box::new(Sphere::from(Point3::from(1.0, 0.0, -1.0),    0.5,   material_right)));

    world
}

#[allow(dead_code)]
fn get_world2() -> HittableList {
    let r = (PI/4.0).cos();

    let material_left =Rc::new(Lambertian::from(&Color::from(0.0,0.0,1.0)));
    let material_right=Rc::new(Lambertian::from(&Color::from(1.0,0.0,0.0)));

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::from(Point3::from(-r, 0.0, -1.0), r, material_left)));
    world.add(Box::new(Sphere::from(Point3::from( r, 0.0, -1.0), r, material_right)));

    world
}
