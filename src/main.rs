mod color;
mod vec3;
mod ray;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use color::Color;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("output/image.ppm");
    let image_width = 256;
    let image_height = 256;

    let mut out = OpenOptions::new().write(true).create(true).open(path)?;

    out.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for i in 0..image_height {
        for j in 0..image_width {
            let c = Color::new_rgb(
                i as f32 / (image_width - 1) as f32,
                j as f32 / (image_height - 1) as f32,
                0.25,
            );
            let v = &c;

            out.write_all(c.to_ppm_string().as_bytes())?;
            out.write_all("\n".as_bytes())?;
        }
    }
    out.flush()?;

    Ok(())
}
