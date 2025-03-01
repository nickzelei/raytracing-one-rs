use crate::{color, hittable, ray, vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: ray::Ray,
        rec: &mut hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool;
}

#[derive(Default)]
pub struct Lambertian {
    albedo: color::Color,
}

impl Lambertian {
    pub fn new(albedo: color::Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: ray::Ray,
        rec: &mut hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal() + vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal();
        }
        *scattered = ray::Ray::new(rec.p(), scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Default)]
pub struct Metal {
    albedo: color::Color,
}

impl Metal {
    pub fn new(albedo: color::Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: ray::Ray,
        rec: &mut hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        let reflected = vec3::reflect(r_in.direction(), rec.normal());
        *scattered = ray::Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        true
    }
}
