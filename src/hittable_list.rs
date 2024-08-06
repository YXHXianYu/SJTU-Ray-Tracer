use crate::hittable::{ Hittable, HitRecord };
use crate::ray::Ray;
use crate::common::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

#[allow(dead_code)]
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if let Some(x) = object.hit(ray, &Interval::from(ray_t.min, closest_so_far)) {
                hit_record = Some(x);
                closest_so_far = hit_record.as_ref().unwrap().t;
            }
        }

        if hit_record.is_some() {
            hit_record
        } else {
            None
        }
    }
}
