use core::f64;

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    // Calc the image height, ensure it's at least 1
    let mut image_height: i64 = ((IMAGE_WIDTH as f64) / aspect_ratio) as i64;
    if image_height < 1 {
        image_height = 1;
    }

    // World
    let mut world = hittable_list::HittableList::new();
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width: f64 = viewport_height * ((IMAGE_WIDTH as f64) / (image_height as f64));
    let camera_center = vec3::Point3::new(0.0, 0.0, 0.0);

    // Calc the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = vec3::Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (IMAGE_WIDTH as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    let viewport_upper_left = camera_center
        - vec3::Vec3::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    println!("P3\n{0} {1}\n255\n", IMAGE_WIDTH, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}\n", (image_height - j));
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (pixel_delta_u * (i as f64)) + (pixel_delta_v * (j as f64));
            let ray_direction = pixel_center - camera_center;

            let r = ray::Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(r, &world);
            color::write_color(pixel_color);
        }
    }
    eprintln!("\rDone.");
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
