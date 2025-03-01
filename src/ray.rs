use crate::vec3;

#[derive(Copy, Clone, Default)]
pub struct Ray {
    orig: vec3::Point3,
    direction: vec3::Vec3,
}

impl Ray {
    pub fn new(origin: vec3::Point3, dir: vec3::Vec3) -> Self {
        Ray {
            orig: origin,
            direction: dir,
        }
    }

    pub fn origin(&self) -> vec3::Point3 {
        self.orig
    }

    pub fn direction(&self) -> vec3::Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> vec3::Point3 {
        self.orig + (self.direction * t)
    }
}
