#![allow(dead_code)]
mod obj_loader;
mod worlds;

use image::ImageError;
use rand::SeedableRng;
use ray_tracing_in_a_weekend::*;
use std::{path::Path, thread};

fn main() -> Result<(), ImageError> {
    let args: Vec<String> = std::env::args().collect();

    let path = Path::new(&args[1]);
    let image_width = args[2].parse::<i32>().unwrap();
    let samples_per_pixel = args[3].parse::<usize>().unwrap();
    let max_depth = args[4].parse::<i32>().unwrap();
    let world_name = args[5].as_str();

    let thread_count = thread::available_parallelism().map_or(1, |x| x.get());
    eprintln!("Using {thread_count} threads.");

    let mut arena = bumpalo::Bump::new();
    let wb = worlds::world_builder::WorldBuilder::new(&mut arena);
    let mut rng = TRng::from_seed([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    let world = match world_name {
        "demo:cornell_box" => worlds::demo_worlds::create_world_cornell_box,
        "demo:defocus_blur" => worlds::demo_worlds::create_world_defocus_blur,
        "demo:simple_plane" => worlds::demo_worlds::create_world_simple_plane,
        "demo:earth_mapped" => worlds::demo_worlds::create_world_earth_mapped,
        "demo:suzanne" => worlds::demo_worlds::create_world_suzanne,
        "demo:moving_spheres" => worlds::demo_worlds::create_world_moving_spheres,
        _ => panic!(),
    }(&wb, &mut rng);
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
        RenderMode::Default,
    );

    eprintln!("Saving image...");
    let bytes = pixels
        .iter()
        .flat_map(|c| c.to_rgb8_gamma2())
        .collect::<Vec<_>>();
    image::save_buffer(
        path,
        &bytes,
        image_size.width as u32,
        image_size.height as u32,
        image::ColorType::Rgb8,
    )
}
