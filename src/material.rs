use crate::common::*;
use crate::hittable::HitRecord;

use rand::Rng;

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

// === Dielectric ===

pub struct Dielectric {
    index_of_refraction: f64,
}

impl Dielectric {
    pub fn from(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction
        }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        hit_record: &HitRecord,
    ) -> Option<(Color, Ray)> {
        let refraction_ratio = if hit_record.front_face { 1.0 / self.index_of_refraction } else { self.index_of_refraction };
        let r = r_in.direction().unit();

        let cos_theta = (-r).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let can_refract = refraction_ratio * sin_theta <= 1.0;

        let refracted = if can_refract && Dielectric::reflectance(cos_theta, refraction_ratio) <= rand::thread_rng().gen_range(0.0..1.0) {
            Vec3::refract(r, hit_record.normal, refraction_ratio)
        } else {
            Vec3::reflect(r, hit_record.normal)
        };

        return Some((
            Color::from(1.0, 1.0, 1.0),
            Ray::from(hit_record.point, refracted),
        ))
    }

}
