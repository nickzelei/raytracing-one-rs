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
    const IMAGE_WIDTH: i64 = 1200;
    const SAMPLES_PER_PIXEL: i64 = 500;
    const MAX_DEPTH: i64 = 50;
    const VFOV: f64 = 20.0;
    const DEFOCUS_ANGLE: f64 = 0.6; // Variantion angle of rays through each pixel
    const FOCUS_DIST: f64 = 10.0; // Distance from camera lookfrom point to plane of perfect focus
    let lookfrom = vec3::Point3::new(13.0, 2.0, 3.0); // Point camera is looking from
    let lookat = vec3::Point3::new(0.0, 0.0, 0.0); // Point camear is looking at
    let vup = vec3::Vec3::new(0.0, 1.0, 0.0); // Camera-relative up direction

    // World
    let mut world = hittable_list::HittableList::new();

    let ground_material = Rc::new(material::Lambertian::new(color::Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_double();
            let center = vec3::Point3::new(
                (a as f64) + 0.9 * utils::random_double(),
                0.2,
                (b as f64) + 0.9 * utils::random_double(),
            );

            if (center - vec3::Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = color::Color::new_random() * color::Color::new_random();
                    let sphere_material = Rc::new(material::Lambertian::new(albedo));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::Color::new_random_bounded(0.5, 1.0);
                    let fuzz = utils::random_double_bounded(0.0, 0.5);
                    let sphere_material = Rc::new(material::Metal::new(albedo, fuzz));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(material::Dielectric::new(1.5));
                    world.add(Box::new(sphere::Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material_1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));

    let material_2 = Rc::new(material::Lambertian::new(color::Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));

    let material_3 = Rc::new(material::Metal::new(color::Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(sphere::Sphere::new(
        vec3::Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

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
