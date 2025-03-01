use std::rc::Rc;

use crate::{interval, material, ray, vec3};

pub struct HitRecord {
    p: vec3::Point3,
    normal: vec3::Vec3,
    mat: Rc<dyn material::Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn p(&self) -> vec3::Point3 {
        self.p
    }

    pub fn set_p(&mut self, input: vec3::Point3) {
        self.p = input;
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn set_t(&mut self, input: f64) {
        self.t = input
    }

    pub fn normal(&self) -> vec3::Vec3 {
        self.normal
    }

    pub fn mat(&self) -> Rc<dyn material::Material> {
        self.mat.clone()
    }

    pub fn set_mat(&mut self, input: Rc<dyn material::Material>) {
        self.mat = input
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

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: vec3::Vec3::default(),
            normal: vec3::Vec3::default(),
            mat: Rc::new(material::Lambertian::default()),
            t: 0.0,
            front_face: false,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: ray::Ray, ray_t: interval::Interval, rec: &mut HitRecord) -> bool;
}
