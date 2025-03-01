use std::rc::Rc;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i64 = 400;
    const SAMPLES_PER_PIXEL: i64 = 100;
    const MAX_DEPTH: i64 = 50;

    // World
    let mut world = hittable_list::HittableList::new();

    let material_ground = Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Metal::new(color::Color::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2)));

    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let cam = camera::Camera::new(aspect_ratio, IMAGE_WIDTH, SAMPLES_PER_PIXEL, MAX_DEPTH);
    cam.render(&world);
}
