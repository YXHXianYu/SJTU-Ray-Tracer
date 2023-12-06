use crate::common::*;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &Color
    ) -> Option<Ray>;
}
