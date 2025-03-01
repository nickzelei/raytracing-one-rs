use crate::{interval, vec3};

pub type Color = vec3::Vec3;

pub fn write_color(pixel_color: Color) {
    let r: f64 = pixel_color.x();
    let g: f64 = pixel_color.y();
    let b: f64 = pixel_color.z();

    // Translates the [0,1] component values to byte range [0,255]
    let intensity = interval::Interval::new(0.000, 0.999);
    let rbyte: i64 = (256.0 * intensity.clamp(r)) as i64;
    let gbyte: i64 = (256.0 * intensity.clamp(g)) as i64;
    let bbyte: i64 = (256.0 * intensity.clamp(b)) as i64;

    println!("{0} {1} {2}", rbyte, gbyte, bbyte);
}
