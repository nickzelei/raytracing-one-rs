use crate::{ray, vec3};

pub struct HitRecord {
    p: vec3::Point3,
    normal: vec3::Vec3,
    t: f64,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, normal: vec3::Vec3, t: f64) -> Self {
        HitRecord { p, normal, t }
    }
}

pub trait Hittable {
    fn hit(&self, r: ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
