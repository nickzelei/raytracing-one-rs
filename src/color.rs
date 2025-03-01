use crate::{interval, vec3};

pub type Color = vec3::Vec3;

pub fn write_color(pixel_color: Color) {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    // Translates the [0,1] component values to byte range [0,255]
    let intensity = interval::Interval::new(0.000, 0.999);
    let rbyte: i64 = (256.0 * intensity.clamp(r)) as i64;
    let gbyte: i64 = (256.0 * intensity.clamp(g)) as i64;
    let bbyte: i64 = (256.0 * intensity.clamp(b)) as i64;

    println!("{0} {1} {2}", rbyte, gbyte, bbyte);
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
