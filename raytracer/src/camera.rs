use indicatif::ProgressBar;
use rand::Rng;
use std::fs::File;

use crate::common::*;
use crate::hittable_list::HittableList;

pub struct Camera {
    // === Hyper Parameters ===
    // aspect_ratio:      f64, // Ratio of image width over height
    image_width :      u32, // Rendered image width in pixel count
    samples_per_pixel: u32, // The number of samples per pixel
    max_depth:         u32, // Maximum number of ray bounces

    // === Derived Parameters ===
    image_height:      u32, // Rendered image height
    center:         Point3, // Camera center
    pixel00_loc:    Point3, // Location of pixel(0, 0)
    pixel_delta_u:    Vec3, // Offset to pixel to the right
    pixel_delta_v:    Vec3, // Offset to pixel to below
}

#[allow(dead_code)]
impl Camera {
    // === Public ===
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Camera {
        
        let image_height = std::cmp::max(1, (image_width as f64 / aspect_ratio) as u32);

        let center = Point3::from(0.0, 0.0,  0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            // aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: &HittableList, out: &mut File, progress: &mut ProgressBar) {

        ppm_header(out, self.image_width, self.image_height);

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::from(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&ray, self.max_depth, world);
                }
                write_color(out, pixel_color, self.samples_per_pixel);
            }
            progress.inc(1);
        }
        progress.finish();
    }

    pub fn image_height(&self) -> u32 {
        self.image_height
    }
    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    // === Private ===
    fn ray_color(ray: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::from(0.0, 0.0, 0.0);
        }

        if let Some(x) = world.hit(ray, &Interval::from(0.001, INFINITY)) {
            let direction = x.normal + Vec3::random_on_hemisphere(x.normal);
            // why x.normal occurs here? because of Lambertian Reflection (b1-9.4)
            return 0.5 * Camera::ray_color(&Ray::from(x.point, direction), depth - 1, world);
        }

        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
    }

    // Get a randomly sampled camera ray for the pixel at location (i, j).
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::from(ray_origin, ray_direction)
    }

    // Generate a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::thread_rng().gen_range(0.0..1.0);
        let py = -0.5 + rand::thread_rng().gen_range(0.0..1.0);
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}
