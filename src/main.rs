fn main() {
    const IMAGE_WIDTH: i64 = 256;
    const IMAGE_HEIGHT: i64 = 256;

    println!("P3\n{0} {1}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let r: f64 = f64(i) / (IMAGE_WIDTH - 1);
            let g: f64 = f64(j) / (IMAGE_HEIGHT - 1);
            let b: f64 = 0.0;

            let ir: i64 = i64(255.999 * r);
            let ig: i64 = i64(255.999 * g);
            let ib: i64 = i64(255.999 * b);

            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
