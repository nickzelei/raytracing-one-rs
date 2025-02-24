mod color;
mod ray;
mod vec3;

fn main() {
    const IMAGE_WIDTH: i64 = 256;
    const IMAGE_HEIGHT: i64 = 256;

    println!("P3\n{0} {1}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {}\n", (IMAGE_HEIGHT - j));
        for i in 0..IMAGE_WIDTH {
            let pixel_color = color::Color::new(
                (i as f64) / (IMAGE_WIDTH as f64 - 1.0),
                (j as f64) / ((IMAGE_HEIGHT as f64) - 1.0),
                0.0,
            );
            color::write_color(pixel_color);
        }
    }
    eprintln!("\rDone.");
}
