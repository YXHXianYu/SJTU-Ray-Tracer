use crate::hittable::{ Hittable, HitRecord };
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.clear();
    }
    
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }

    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut hit_record = HitRecord::new();
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if let Some(x) = object.hit(ray, ray_tmin, closest_so_far) {
                hit_anything = true;
                hit_record = x;
                closest_so_far = hit_record.t;
            }
        }

        if hit_anything {
            Some(hit_record)
        } else {
            None
        }
    }
}
