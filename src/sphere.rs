use crate::{hittable, ray, vec3};

pub struct Sphere {
    center: vec3::Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: vec3::Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl hittable::Hittable for Sphere {
    fn hit(
        &self,
        r: ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let oc = self.center - r.origin();
        let a = r.direction().length_squared();
        let h = vec3::dot(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        let t = root;
        let p = r.at(t);
        // let normal = (p - self.center) / self.radius;
        let outward_normal = (p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
