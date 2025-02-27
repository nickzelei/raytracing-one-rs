use crate::hittable;

pub struct HittableList {
    objects: Vec<Box<dyn hittable::Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn hittable::Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects = Vec::new();
    }
}

impl hittable::Hittable for HittableList {
    fn hit(
        &self,
        r: crate::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        rec: &mut hittable::HitRecord,
    ) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            let mut temp_rec = hittable::HitRecord::default();
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t();
                *rec = temp_rec;
            }
        }

        hit_anything
    }
}
