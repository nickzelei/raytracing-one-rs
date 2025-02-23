fn main() {
    const IMAGE_WIDTH: i64 = 256;
    const IMAGE_HEIGHT: i64 = 256;

    println!("P3\n{0} {1}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r: f64 = (i as f64) / ((IMAGE_WIDTH as f64) - 1.0);
            let g: f64 = (j as f64) / ((IMAGE_HEIGHT as f64) - 1.0);
            let b: f64 = 0.0;

            let ir: i64 = (255.999 * r) as i64;
            let ig: i64 = (255.999 * g) as i64;
            let ib: i64 = (255.999 * b) as i64;

            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
