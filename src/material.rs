use crate::{color, hittable, ray, utils, vec3};

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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: color::Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        let mut reflected = vec3::reflect(r_in.direction(), rec.normal());
        reflected = vec3::unit_vector(reflected) + (self.fuzz * vec3::random_unit_vector());
        *scattered = ray::Ray::new(rec.p(), reflected);
        *attenuation = self.albedo;
        vec3::dot(scattered.direction(), rec.normal()) > 0.0
    }
}

#[derive(Default)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + self.refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: ray::Ray,
        rec: &mut hittable::HitRecord,
        attenuation: &mut color::Color,
        scattered: &mut ray::Ray,
    ) -> bool {
        *attenuation = color::Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = vec3::unit_vector(r_in.direction());

        let cos_theta = vec3::dot(-unit_direction, rec.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || self.reflectance(cos_theta, ri) > utils::random_double() {
                vec3::reflect(unit_direction, rec.normal())
            } else {
                vec3::refract(unit_direction, rec.normal(), ri)
            };
        *scattered = ray::Ray::new(rec.p(), direction);
        true
    }
}
