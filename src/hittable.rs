use std::sync::Arc;

use crate::common::*;
use crate::ray::Ray;
use crate::vec3::{ Vec3, Point3 };
use crate::material::Material;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub front_face: bool,
    pub t: f64,
    pub material: Arc<dyn Material + Send + Sync>,
}

impl HitRecord {
    pub fn from(
        point: Point3,
        ray: &Ray, 
        outward_normal: &Vec3,
        t: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> HitRecord {

        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };

        HitRecord {
            point,
            normal,
            front_face,
            t,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}
