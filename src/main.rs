mod color;
mod ray;
mod vec3;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use color::Color;
use ray::Ray;
use vec3::Dir3;
use vec3::Point3;

fn lerp<T: std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    a * (1.0 - t) + b * t
}

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (Dir3::dot(Dir3::UNIT_Y, unit_direction) + 1.0);
    let ground_color = Color::new_rgb(0.5, 0.7, 1.0);
    let sky_color = Color::new_rgb(1.0, 1.0, 1.0);

    return lerp(sky_color, ground_color, t);
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("output/image.ppm");
    let image_width = 400;
    let image_height = 225;
    let aspect_ratio = image_width as f32 / image_height as f32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::ZERO;
    let horizontal = viewport_width * Dir3::UNIT_X;
    let vertical = viewport_height * Dir3::UNIT_Y;
    let depth = focal_length * Dir3::UNIT_Z;
    let upper_left_corner = horizontal * -0.5 + vertical * 0.5 - depth;

    let camera = |ix: i32, iy: i32| -> Ray {
        let ify = iy as f32 / (image_height - 1) as f32;
        let ifx = ix as f32 / (image_width - 1) as f32;
        Ray {
            origin,
            direction: upper_left_corner + ifx * horizontal - ify * vertical,
        }
    };

    let mut out = OpenOptions::new().write(true).create(true).open(path)?;

    out.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for i in 0..image_height {
        for j in 0..image_width {
            let ray = camera(j, i);
            let c = ray_color(ray);

            out.write_all(c.to_ppm_string().as_bytes())?;
            out.write_all("\n".as_bytes())?;
        }
    }
    out.flush()?;

    Ok(())
}
