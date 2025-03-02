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
    const VFOV: f64 = 90.0;

    // World
    let mut world = hittable_list::HittableList::new();

    let material_left = Rc::new(material::Lambertian::new(color::Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(material::Lambertian::new(color::Color::new(1.0, 0.0, 0.0)));
    // let material_center = Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(material::Dielectric::new(1.50));
    // let material_bubble = Rc::new(material::Dielectric::new(1.00 / 1.50));
    // let material_right = Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 1.0));

    let r = (utils::PI / 4.0).cos();
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));
    // world.add(Box::new(sphere::Sphere::new(
    //     vec3::Point3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     vec3::Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     vec3::Point3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     material_bubble,
    // )));
    // world.add(Box::new(sphere::Sphere::new(
    //     vec3::Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));

    let cam = camera::Camera::new(
        aspect_ratio,
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        VFOV,
    );
    cam.render(&world);
}
