use crate::{interval, ray, vec3};

#[derive(Debug, Default)]
pub struct HitRecord {
    p: vec3::Point3,
    normal: vec3::Vec3,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: vec3::Point3, normal: vec3::Vec3, t: f64, front_face: bool) -> Self {
        Self {
            p,
            normal,
            t,
            front_face,
        }
    }

    pub fn p(&self) -> vec3::Point3 {
        self.p
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn normal(&self) -> vec3::Vec3 {
        self.normal
    }

    pub fn set_face_normal(&mut self, r: ray::Ray, outward_normal: vec3::Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the outward_normal is assumed to have unit length.
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal;
        } else {
            self.normal = -outward_normal;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: ray::Ray, ray_t: interval::Interval, rec: &mut HitRecord) -> bool;
}
