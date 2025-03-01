mod camera;
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

    let cam = camera::Camera::new(aspect_ratio, IMAGE_WIDTH);
    cam.render(&world);
}
