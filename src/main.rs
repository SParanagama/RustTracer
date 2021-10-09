use std::fs::File;
use std::io::prelude::*;

fn main() {

    let image_width: i32 = 256;
    let image_height: i32 = 256;

    let mut image_content = String::new();
    image_content.push_str("P3\n");
    image_content.push_str(&format!("{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}\n", {j});
        for i in 0..image_width {
            let r = i as f64 / (image_width-1) as f64;
            let g = j as f64 / (image_height-1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            image_content.push_str(&format!("{} {} {}\n", ir, ig, ib));
        }
    }


    let result = File::create("image.ppm")
    .and_then(|mut file| file.write_all(image_content.as_bytes()));

    if result.is_ok() {
        println!("File wrote successfully!");
    } else {
        println!("Error writing image file");
    }
}
