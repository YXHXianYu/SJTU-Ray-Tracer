use crate::common::*;
use crate::hittable::HitRecord;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)>;
}

// === Lambertian (Diffuse) ===

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn from(color: &Color) -> Lambertian {
        Lambertian {
            albedo: *color,
        }
    }
}

impl Material for Lambertian {
    // The return value is Option<(Color, Ray)>
    // 'Color' means attenuation
    // 'Ray' means scattered ray
    fn scatter(
        &self,
        _r_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.is_zero() {
            scatter_direction = hit_record.normal;
        }

        Some((
            self.albedo,
            Ray::from(hit_record.point, scatter_direction)
        ))
    }
}

// === Metal (Mirror Reflection) ===

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn from(color: &Color, fuzz: f64) -> Metal {
        Metal {
            albedo: *color,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r_in.direction().unit(), hit_record.normal);
        let scattered = Ray::from(hit_record.point, reflected + self.fuzz * Vec3::random_unit_vector());
        
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((
                self.albedo,
                scattered
            ))
        } else {
            None
        }
    }
}

