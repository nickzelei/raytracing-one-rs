use crate::{color, hittable, interval, ray, vec3};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    image_width: i64,

    image_height: i64,         // Rendered image height
    center: vec3::Point3,      // Camera center
    pixel00_loc: vec3::Point3, // Location of pixel 0, 0
    pixel_delta_u: vec3::Vec3, // Offset to pixel to the right
    pixel_delta_v: vec3::Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i64) -> Self {
        let image_height = calculate_image_height(image_width, aspect_ratio);
        let center = vec3::Point3::new(0.0, 0.0, 0.0);
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width: f64 = viewport_height * ((image_width as f64) / (image_height as f64));

        // Calc the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vec3::Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        let viewport_upper_left =
            center - vec3::Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Self {
            image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }

    pub fn render(&self, world: &dyn hittable::Hittable) {
        println!("P3\n{0} {1}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}\n", (self.image_height - j));
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (self.pixel_delta_u * (i as f64))
                    + (self.pixel_delta_v * (j as f64));
                let ray_direction = pixel_center - self.center;

                let r = ray::Ray::new(self.center, ray_direction);
                let pixel_color = ray_color(r, world);
                color::write_color(pixel_color);
            }
        }
        eprintln!("\rDone.");
    }
}

fn ray_color(r: ray::Ray, world: &dyn hittable::Hittable) -> color::Color {
    let mut rec = hittable::HitRecord::default();
    if world.hit(r, interval::Interval::new(0.0, f64::INFINITY), &mut rec) {
        return (rec.normal() + color::Color::new(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;
    return (color::Color::new(1.0, 1.0, 1.0) * (1.0 - a)) + (color::Color::new(0.5, 0.7, 1.0) * a);
}

fn calculate_image_height(image_width: i64, aspect_ratio: f64) -> i64 {
    let height = ((image_width as f64) / aspect_ratio) as i64;
    if height < 1 {
        return 1;
    }
    height
}
