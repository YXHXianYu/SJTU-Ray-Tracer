use crate::hittable::{ Hittable, HitRecord };
use crate::vec3::Point3;
use crate::ray::Ray;

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn from(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
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
        let mut hit_record = HitRecord::new();
        if ray_tmin < r1 && r1 < ray_tmax {
            hit_record.t = r1;
        } else if ray_tmin < r2 && r2 < ray_tmax {
            hit_record.t = r2;
        } else {
            return None;
        }
        
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);

        Some(hit_record)
    }
}
