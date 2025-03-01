use crate::{hittable, interval, ray, vec3};

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
    fn hit(&self, r: ray::Ray, ray_t: interval::Interval, rec: &mut hittable::HitRecord) -> bool {
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
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.set_t(root);
        rec.set_p(r.at(rec.t()));
        let outward_normal = (rec.p() - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
