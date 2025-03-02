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
    const VFOV: f64 = 20.0;
    const DEFOCUS_ANGLE: f64 = 10.0; // Variantion angle of rays through each pixel
    const FOCUS_DIST: f64 = 3.4; // Distance from camera lookfrom point to plane of perfect focus

    // World
    let mut world = hittable_list::HittableList::new();

    let material_ground = Rc::new(material::Lambertian::new(color::Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(material::Lambertian::new(color::Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(material::Dielectric::new(1.50));
    let material_bubble = Rc::new(material::Dielectric::new(1.00 / 1.50));
    let material_right = Rc::new(material::Metal::new(color::Color::new(0.8, 0.6, 0.2), 1.0));

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
        vec3::Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let lookfrom = vec3::Point3::new(-2.0, 2.0, 1.0); // Point camera is looking from
    let lookat = vec3::Point3::new(0.0, 0.0, -1.0); // Point camear is looking at
    let vup = vec3::Vec3::new(0.0, 1.0, 0.0); // Camera-relative up direction

    let cam = camera::Camera::new(
        aspect_ratio,
        IMAGE_WIDTH,
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        VFOV,
        lookfrom,
        lookat,
        vup,
        DEFOCUS_ANGLE,
        FOCUS_DIST,
    );
    cam.render(&world);
}
