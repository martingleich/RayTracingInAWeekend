use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("output/image.ppm");
    let image_width = 256;
    let image_height = 256;

    let mut out = OpenOptions::new().write(true).create(true).open(path)?;

    out.write_all(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for i in 0..image_height {
        for j in 0..image_width {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let ir = (r * 255.999) as i32;
            let ig = (g * 255.999) as i32;
            let ib = (b * 255.999) as i32;

            out.write_all(&format!("{ir} {ig} {ib}\n").as_bytes())?;
        }
    }
    out.flush()?;

    Ok(())
}
