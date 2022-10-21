mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod ray;
mod size2i;
mod vec2;
mod vec3;

use std::{
    path::Path,
    sync::mpsc,
    thread::{self},
};

use camera::Camera;
use color::Color;
use hittable::{Hittable, HittableList, Sphere};

use image::ImageError;
use material::Material;
use rand::{distributions::Uniform, Rng, SeedableRng};
use ray::Ray;
use size2i::Size2i;
use vec2::Vec2f;
use vec3::{Dir3, Point3};


fn sky_color(ray: &Ray) -> Color {
    let t = 0.5 * (Dir3::dot(Dir3::UP, ray.direction) + 1.0);
    let ground_color = Color::new_rgb(0.5, 0.7, 1.0);
    let sky_color = Color::new_rgb(1.0, 1.0, 1.0);
    math::lerp(sky_color, ground_color, t)
}

fn ray_color<THit: Hittable, TRng: rand::Rng>(
    ray: &Ray,
    world: &THit,
    rng: &mut TRng,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::BLACK;
    }

    if let Some(interaction) = world.hit(ray, &(0.0001..f32::INFINITY)) {
        if let Some((attentuation, scattered)) =
            interaction.material.scatter(ray, &interaction, rng)
        {
            Color::convolution(attentuation, ray_color(&scattered, world, rng, depth - 1))
        } else {
            Color::BLACK
        }
    } else {
        sky_color(ray)
    }
}

fn main() -> Result<(), ImageError> {
    let path = Path::new("output/image.png");
    let image_size = Size2i::new(800, 450);
    let samples_per_pixel = 100;
    let max_depth = 50;
    let thread_count = thread::available_parallelism().map_or(4, |x| x.get());
    eprintln!("Using {thread_count} threads.");

    let viewport_width = 1.2;
    let viewport_height = image_size.aspect_ratio() * viewport_width;
    let camera = Camera::new(
        viewport_width,
        viewport_height,
        1.0,
        Point3::ORIGIN + Dir3::BACKWARD * 3.0,
        Dir3::UP,
        Dir3::FORWARD,
    );

    let world = {
        let mut world = HittableList::new();
        let material_ground = Material::Lambert {
            albedo: Color::new_rgb(0.8, 0.8, 0.0),
        };
        let material_center = Material::Lambert {
            albedo: Color::new_rgb(0.7, 0.3, 0.3),
        };
        let material_left = Material::Metal {
            albedo: Color::new_rgb(0.6, 0.6, 0.8),
            fuzz: 0.0,
        };
        let material_right = Material::Metal {
            albedo: Color::new_rgb(0.8, 0.6, 0.2),
            fuzz: 1.0,
        };
        world.push(Sphere::new(
            Point3::ORIGIN + Dir3::DOWN * 100.5,
            100.0,
            material_ground,
        ));
        world.push(Sphere::new(
            Point3::ORIGIN + Dir3::FORWARD,
            0.5,
            material_center,
        ));
        world.push(Sphere::new(
            Point3::ORIGIN + Dir3::LEFT + Dir3::FORWARD,
            0.5,
            material_left,
        ));
        world.push(Sphere::new(
            Point3::ORIGIN + Dir3::RIGHT + Dir3::FORWARD,
            0.5,
            material_right,
        ));
        world
    };

    let pixels = render(
        image_size,
        thread_count,
        samples_per_pixel,
        max_depth,
        &camera,
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

fn render(
    image_size: Size2i,
    thread_count: usize,
    samples_per_pixel: usize,
    max_depth: i32,
    camera: &Camera,
    world: &HittableList,
) -> Vec<Color> {
    eprintln!("Start rendering...");
    let pixel_sample_distr = Uniform::new(
        Vec2f { x: 0.0, y: 0.0 },
        Vec2f {
            x: 1.0 / (image_size.width - 1) as f32,
            y: 1.0 / (image_size.height - 1) as f32,
        },
    );
    let (tx, rx) = mpsc::channel::<(usize, usize)>();

    let total_work = samples_per_pixel * image_size.count();
    let mut done_work = vec![0; thread_count];
    let report_update_thread = thread::spawn(move || loop {
        match rx.recv() {
            Ok((id, done)) => {
                done_work[id] = done;
                let total_done = done_work.iter().sum::<usize>();
                let done_percent = (100 * total_done) / total_work;
                eprint!("\r{done_percent} %");
            }
            Err(_) => {
                eprintln!("\rRendering done");
                return;
            }
        };
    });

    let one_work_step = total_work / (thread_count * 100);
    let pixel_sample_distr_ref = &pixel_sample_distr;
    let mut rng = rand::rngs::StdRng::from_entropy();
    let planes = thread::scope(|s| {
        let whole_sample_per_pixel = samples_per_pixel / thread_count;
        let remaining_samples_per_pixel = samples_per_pixel % thread_count;
        (0..thread_count)
            .map(|thread_id| {
                let real_samples_per_pixel =
                    whole_sample_per_pixel + (thread_id < remaining_samples_per_pixel) as usize;
                let local_tx = tx.clone();

                let mut sub_rng = rand_xoshiro::Xoroshiro128PlusPlus::from_rng(&mut rng).unwrap();
                let per_pixel = move |fpix: Vec2f| {
                    (0..real_samples_per_pixel)
                        .map(|_| {
                            let pix = fpix + sub_rng.sample(pixel_sample_distr_ref);
                            let ray = camera.ray(pix);
                            ray_color(&ray, world, &mut sub_rng, max_depth)
                        })
                        .sum::<Color>()
                        / real_samples_per_pixel as f32
                };

                let mut count = 0;
                let update = move |fpix: Color| {
                    if count % one_work_step == 0 {
                        local_tx.send((thread_id, count)).unwrap();
                    }
                    count += real_samples_per_pixel;
                    fpix
                };

                s.spawn(move || {
                    image_size
                        .iterf()
                        .map(per_pixel)
                        .map(update) // Sideeffect
                        .collect::<Vec<_>>()
                })
            })
            .map(|t| t.join().unwrap())
            .collect::<Vec<_>>()
    });
    drop(tx); // Drop the final sender

    report_update_thread.join().unwrap();

    eprintln!("Merging threads...");
    merge_planes(image_size, planes, thread_count)
}

fn merge_planes(image_size: Size2i, planes: Vec<Vec<Color>>, thread_count: usize) -> Vec<Color> {
    let mut pixels: Vec<Color> = vec![Color::BLACK; image_size.count()];
    for plane in planes {
        for (p, i) in plane.iter().enumerate() {
            pixels[p] += *i;
        }
    }
    for pix in &mut pixels {
        *pix = *pix / (thread_count as f32);
    }
    pixels
}
