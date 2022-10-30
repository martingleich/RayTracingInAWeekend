#![allow(dead_code)]
mod rendering;
mod worlds;

use image::ImageError;
use ray_tracing_in_a_weekend::*;
use std::{path::Path, thread};

fn main() -> Result<(), ImageError> {
    let args: Vec<String> = std::env::args().collect();

    let path = Path::new(&args[1]);
    let image_width = args[2].parse::<i32>().unwrap(); // 800
    let samples_per_pixel = args[3].parse::<usize>().unwrap(); // 100
    let max_depth = args[4].parse::<i32>().unwrap(); // 50

    let thread_count = thread::available_parallelism().map_or(1, |x| x.get());
    eprintln!("Using {thread_count} threads.");

    let mut arena = bumpalo::Bump::new();
    let world = worlds::create_world_cornell_box(
        &mut arena,
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
    );
    let image_size = Size2i::new(
        image_width,
        (image_width as f32 * world.camera.aspect_ratio()) as i32,
    );

    let pixels = crate::rendering::render(
        image_size,
        thread_count,
        samples_per_pixel,
        max_depth,
        &world,
    );

    eprintln!("Saving image...");
    let bytes = pixels.iter().flat_map(|c| c.to_rgb8()).collect::<Vec<_>>();
    image::save_buffer(
        path,
        &bytes,
        image_size.width as u32,
        image_size.height as u32,
        image::ColorType::Rgb8,
    )
}
