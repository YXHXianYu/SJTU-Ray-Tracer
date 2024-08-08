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

use std::env;
use std::fs::File;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

use indicatif::ProgressBar;
use console::style;
use image::{ ImageBuffer, RgbImage };
use rand::Rng;

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
        samples_per_pixel: 500,
        max_depth: 50,
        image_width: 1920,
        // 400 * 400 * 9/16 ==> 1080 * 1920

        camera_position: Vec3::from(13.0, 2.0, 3.0),
        look_at: Vec3::from(0.0, 0.0, 0.0),
        vfov: 20.0,

        defocus_angle: 0.6,
        focus_dist: 10.0,

        ..Default::default()
    });
    let progress = progress_bar_setup(camera.image_height() * camera.image_width());

    let mut img: RgbImage = ImageBuffer::new(camera.image_width(), camera.image_height());

    let world = get_world3();

    let t = Instant::now();
    camera.render(Arc::new(world), &mut img, Arc::new(Mutex::new(progress)));
    println!("done! cost: {:?}", t.elapsed());

    println!(
        "Ouput image as \"{}\"",
        style(path).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    match output_image.write_to(&mut output_file, image::ImageFormat::Jpeg) {
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

fn progress_bar_setup(total: u32) -> ProgressBar {
    if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((total) as u64)
    }
}

#[allow(dead_code)]
fn get_world1() -> HittableList {
    let material_ground = Arc::new(Lambertian::from(&Color::from(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from(&Color::from(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::from(1.5));
    let material_right  = Arc::new(Metal::from(&Color::from(0.8, 0.6, 0.2), 0.0));

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

    let material_left =Arc::new(Lambertian::from(&Color::from(0.0,0.0,1.0)));
    let material_right=Arc::new(Lambertian::from(&Color::from(1.0,0.0,0.0)));

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::from(Point3::from(-r, 0.0, -1.0), r, material_left)));
    world.add(Box::new(Sphere::from(Point3::from( r, 0.0, -1.0), r, material_right)));

    world
}

#[allow(dead_code)]
fn get_world3() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::from(&Color::from(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::from(Point3::from(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::from(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Vec3::from(4.0, 0.2, 0.0)).abs2() > 0.81 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                    let mat = Arc::new(Lambertian::from(&albedo));
                    world.add(Box::new(Sphere::from(center, 0.2, mat)));

                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let mat = Arc::new(Metal::from(&albedo, fuzz));
                    world.add(Box::new(Sphere::from(center, 0.2, mat)));

                } else {
                    // glass
                    let mat = Arc::new(Dielectric::from(1.5));
                    world.add(Box::new(Sphere::from(center, 0.2, mat)));

                }
            }
        }
    }

    let mat1 = Arc::new(Dielectric::from(1.5));
    world.add(Box::new(Sphere::from(Vec3::from(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Arc::new(Lambertian::from(&Color::from(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::from(Vec3::from(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Arc::new(Metal::from(&Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::from(Vec3::from(4.0, 1.0, 0.0), 1.0, mat3)));


    world
}
