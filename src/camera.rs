use crate::{
    color, hittable, interval, ray, utils,
    vec3::{self, random_unit_vector},
};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    image_width: i64,

    image_height: i64,         // Rendered image height
    center: vec3::Point3,      // Camera center
    pixel00_loc: vec3::Point3, // Location of pixel 0, 0
    pixel_delta_u: vec3::Vec3, // Offset to pixel to the right
    pixel_delta_v: vec3::Vec3, // Offset to pixel below
    samples_per_pixel: i64,    // Count of random samples for each pixel
    pixel_samples_scale: f64,  // Color scale factor for a sum of pixel samples
    max_depth: i64,            // max number of ray bounces into scene
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i64,
        samples_per_pixel: i64,
        max_depth: i64,
    ) -> Self {
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

        let pixel_samples_scale = 1.0 / (samples_per_pixel as f64);

        Self {
            image_width,
            image_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
        }
    }

    pub fn render(&self, world: &dyn hittable::Hittable) {
        println!("P3\n{0} {1}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}\n", (self.image_height - j));
            for i in 0..self.image_width {
                let mut pixel_color = color::Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(r, self.max_depth, world);
                }
                color::write_color(self.pixel_samples_scale * pixel_color);
            }
        }
        eprintln!("\rDone.");
    }

    fn get_ray(&self, i: i64, j: i64) -> ray::Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.x()) * self.pixel_delta_u)
            + (((j as f64) + offset.y()) * self.pixel_delta_v);
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        ray::Ray::new(ray_origin, ray_direction)
    }
}

fn sample_square() -> vec3::Vec3 {
    // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
    vec3::Vec3::new(
        utils::random_double() - 0.5,
        utils::random_double() - 0.5,
        0.0,
    )
}

fn ray_color(r: ray::Ray, depth: i64, world: &dyn hittable::Hittable) -> color::Color {
    // if we've exceeded the limit, no more light is gathered
    if depth <= 0 {
        return color::Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = hittable::HitRecord::default();
    if world.hit(r, interval::Interval::new(0.001, f64::INFINITY), &mut rec) {
        let direction = rec.normal() + random_unit_vector();
        return 0.5 * ray_color(ray::Ray::new(rec.p(), direction), depth - 1, world);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return ((1.0 - a) * color::Color::new(1.0, 1.0, 1.0)) + (a * color::Color::new(0.5, 0.7, 1.0));
}

fn calculate_image_height(image_width: i64, aspect_ratio: f64) -> i64 {
    let height = ((image_width as f64) / aspect_ratio) as i64;
    if height < 1 {
        return 1;
    }
    height
}
