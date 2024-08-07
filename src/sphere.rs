use std::sync::Arc;

use crate::common::*;
use crate::hittable::{ Hittable, HitRecord };
use crate::vec3::Point3;
use crate::ray::Ray;
use crate::material::Material;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn from(
        center: Point3,
        radius: f64,
        material: Arc<dyn Material + Send + Sync>,
    ) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().abs2();
        let half_b = oc.dot(&ray.direction());
        let c = oc.abs2() - self.radius * self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 { return None; }
        let sqrtd = discriminant.sqrt();

        // first  check minz intersection
        // second check maxz intersection
        let r1 = (-half_b - sqrtd) / a;
        let r2 = (-half_b + sqrtd) / a;
        let t;
        if ray_t.surrounds(r1) {
            t = r1;
        } else if ray_t.surrounds(r2) {
            t = r2;
        } else {
            return None;
        }
        
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let hit_record = HitRecord::from(
            point,
            ray,
            &outward_normal,
            t,
            Arc::clone(&self.material)
        );

        Some(hit_record)
    }
}
