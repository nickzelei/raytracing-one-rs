use crate::{ray, vec3};

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
    fn hit(&self, r: ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}
