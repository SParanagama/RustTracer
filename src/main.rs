use std::fs::File;
use std::io::prelude::*;

mod math;
use math::*;
mod utils;
use utils::*;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2 as f64 - vertical / 2 as f64 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    let mut image_content = String::new();
    image_content.push_str("P3\n");
    image_content.push_str(&format!("{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {}\n", { j });
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let color = ray_color(&r);
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

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    math::lerp(&Color::new(1.0, 1.0, 1.0), &Color::new(0.5, 0.7, 1.0), t)
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    return discriminant > 0.0;
}
