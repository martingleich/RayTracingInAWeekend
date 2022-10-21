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
        // Sky
        let t = 0.5 * (Dir3::dot(Dir3::UP, ray.direction) + 1.0);
        let ground_color = Color::new_rgb(0.5, 0.7, 1.0);
        let sky_color = Color::new_rgb(1.0, 1.0, 1.0);

        math::lerp(sky_color, ground_color, t)
    }
}

fn main() -> Result<(), ImageError> {
    let path = Path::new("output/image.png");
    let image_size = Size2i::new(400, 225);
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
    samples_per_pixel: i32,
    max_depth: i32,
    camera: &Camera,
    world: &HittableList,
) -> Vec<Color> {
    let pixel_sample_distr = Uniform::new(
        Vec2f { x: 0.0, y: 0.0 },
        Vec2f {
            x: 1.0 / (image_size.width - 1) as f32,
            y: 1.0 / (image_size.height - 1) as f32,
        },
    );
    let pixel_sample_distr_ref = &pixel_sample_distr;
    let mut rng = rand::rngs::StdRng::from_entropy();
    let planes = thread::scope(|s| {
        let real_sample_per_pixel = samples_per_pixel / thread_count as i32;
        (0..thread_count)
            .map(|_| {
                let mut sub_rng = rand_xoshiro::Xoroshiro128PlusPlus::from_rng(&mut rng).unwrap();
                let per_pixel = move |fpix: Vec2f| {
                    (0..real_sample_per_pixel)
                        .map(|_| {
                            let pix = fpix + sub_rng.sample(pixel_sample_distr_ref);
                            let ray = camera.ray(pix);
                            ray_color(&ray, world, &mut sub_rng, max_depth)
                        })
                        .sum::<Color>()
                        / real_sample_per_pixel as f32
                };

                s.spawn(move || image_size.iterf().map(per_pixel).collect::<Vec<_>>())
            })
            .map(|t| t.join().unwrap())
            .collect::<Vec<_>>()
    });

    merge_planes(image_size, planes, thread_count)
}

fn merge_planes(image_size: Size2i, result: Vec<Vec<Color>>, thread_count: usize) -> Vec<Color> {
    let mut pixels: Vec<Color> = vec![Color::BLACK; image_size.count() as usize];
    for plane in result {
        for (p, i) in plane.iter().enumerate() {
            pixels[p] += *i;
        }
    }
    for pix in &mut pixels {
        *pix = *pix / (thread_count as f32);
    }
    pixels
}
