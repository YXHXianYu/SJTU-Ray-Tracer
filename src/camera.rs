use indicatif::ProgressBar;
use rand::Rng;

use image::RgbImage;

use crate::common::*;
use crate::hittable_list::HittableList;

#[allow(dead_code)]
pub struct Camera {
    // === Hyper Parameters ===
    image_width :      u32, // Rendered image width in pixel count
    samples_per_pixel: u32, // The number of samples per pixel
    max_depth:         u32, // Maximum number of ray bounces

    // === Derived Parameters ===
    image_height:      u32, // Rendered image height
    position:          Point3, // Camera position
    pixel00_loc:       Point3, // Location of pixel(0, 0)
    pixel_delta_u:     Vec3, // Offset to pixel to the right
    pixel_delta_v:     Vec3, // Offset to pixel to below

    right_vec: Vec3,
    up_vec: Vec3,
    backward_vec: Vec3,

    defocus_angle:  f64,
    focus_dist:     f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

pub struct CameraCreateInfo {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub vfov: f64,

    pub camera_position: Vec3,
    pub look_at: Vec3,
    pub world_up: Vec3,

    pub defocus_angle: f64,
    pub focus_dist:    f64,
}

impl Default for CameraCreateInfo {
    fn default() -> Self {
        CameraCreateInfo{
            samples_per_pixel: 10,
            max_depth: 10,
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            vfov: 90.0,

            camera_position: Vec3::new(),
            look_at: Vec3::from(0.0, 0.0, -1.0),
            world_up: Vec3::from(0.0, 1.0, 0.0),

            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

#[allow(dead_code)]
impl Camera {
    // === Public ===
    pub fn new(info: CameraCreateInfo) -> Camera {
        
        let image_width = info.image_width;
        let image_height = std::cmp::max(1, (image_width as f64 / info.aspect_ratio) as u32);

        // let position = Point3::from(0.0, 0.0,  0.0);
        let position = info.camera_position;
        let defocus_angle = info.defocus_angle;
        let focus_dist = info.focus_dist;

        let theta = degrees_to_radians(info.vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focus_dist; // MARK: ??
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let backward_vec = (info.camera_position - info.look_at).unit();
        let right_vec = info.world_up.cross(&backward_vec).unit();
        let up_vec = backward_vec.cross(&right_vec);

        // TODO: Why dot right_vec & up_vec?
        let viewport_u = viewport_width * right_vec;
        let viewport_v = viewport_height * -up_vec;

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = position - (focus_dist * backward_vec) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let samples_per_pixel = info.samples_per_pixel;
        let max_depth = info.max_depth;

        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle / 2.0).tan();
        let defocus_disk_u = right_vec * defocus_radius;
        let defocus_disk_v = up_vec * defocus_radius;

        Camera {
            // aspect_ratio,
            image_width,
            image_height,
            position,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,

            backward_vec,
            right_vec,
            up_vec,

            focus_dist,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HittableList, img: &mut RgbImage, progress: &mut ProgressBar) {

        // ppm_header(out, self.image_width, self.image_height);

        for j in (0..self.image_height).rev() {
            for i in 0..self.image_width {
                let mut pixel_color = Color::from(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(&ray, self.max_depth, world);
                }
                // write_color(out, pixel_color, self.samples_per_pixel);
                let pixel = img.get_pixel_mut(i, j);
                *pixel = image::Rgb(transform_color(pixel_color, self.samples_per_pixel));
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
            if let Some((attenuation, scattered)) = x.material.scatter(ray, &x) {
                return attenuation * Camera::ray_color(&scattered, depth-1, world);
            } else {
                return Color::from(0.0, 0.0, 0.0);
            }

            // let direction = x.normal + Vec3::random_on_hemisphere(x.normal);
            // // why x.normal occurs here? because of Lambertian Reflection (book 1 - 9.4)
            // return 0.5 * Camera::ray_color(&Ray::from(x.point, direction), depth - 1, world);
        }

        let unit_direction = ray.direction().unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
    }

    // Get a randomly sampled camera ray for the pixel at location (i, j).
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center = self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle < 0.0 { self.position } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;

        Ray::from(ray_origin, ray_direction)
    }

    // Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.position + p[0] * self.defocus_disk_u + p[1] * self.defocus_disk_v
    }

    // Generate a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rand::thread_rng().gen_range(0.0..1.0);
        let py = -0.5 + rand::thread_rng().gen_range(0.0..1.0);
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
}
