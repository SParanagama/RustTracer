use std::fs::File;
use std::io::prelude::*;

mod math;
mod utils;

fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    let mut image_content = String::new();
    image_content.push_str("P3\n");
    image_content.push_str(&format!("{} {}\n255\n", image_width, image_height));

    for j in (0..image_height).rev() {
        println!("Scanlines remaining: {}\n", { j });
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25;

            let color = utils::color::Color::new(r, g, b);
            color.write_color(&mut image_content);
        }
    }

    let result =
        File::create("image.ppm").and_then(|mut file| file.write_all(image_content.as_bytes()));

    if result.is_ok() {
        println!("File wrote successfully!");
    } else {
        println!("Error writing image file");
    }
}
