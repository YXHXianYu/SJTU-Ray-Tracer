use crate::ray::Ray;
use crate::vec3::{ Vec3, Point3 };

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Point3::new(),
            normal: Vec3::new(),
            front_face: false,
            t: 0.0
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
