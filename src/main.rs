mod camera;
mod color;
mod hittable;
mod math;
mod ray;
mod size2i;
mod vec2;
mod vec3;

use std::{
    fmt,
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
};

use camera::Camera;
use color::Color;
use hittable::{Hittable, HittableList, Sphere};

use rand::{distributions::Uniform, rngs::ThreadRng, Rng};
use rand_distr::{Distribution, UnitSphere};
use ray::Ray;
use size2i::Size2i;
use vec2::Vec2f;
use vec3::{Dir3, Point3};

fn ray_color<THit: Hittable, TRng: rand::Rng>(
    ray: Ray,
    world: &THit,
    rng: &mut TRng,
    depth: i32,
) -> Color {
    if depth <= 0 {
        return Color::BLACK;
    }
    if let Some(interaction) = world.hit(ray, 0.0001, f32::INFINITY) {
        let direction = (interaction.normal + Dir3::new_from_arr(UnitSphere.sample(rng))).unit();
        let new_ray = Ray::new(
            interaction.position,
            direction,
        );
        return 0.5 * ray_color(new_ray, world, rng, depth - 1);
    }
    let t = 0.5 * (Dir3::dot(Dir3::UP, ray.direction) + 1.0);
    let ground_color = Color::new_rgb(0.5, 0.7, 1.0);
    let sky_color = Color::new_rgb(1.0, 1.0, 1.0);

    return math::lerp(sky_color, ground_color, t);
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("output/image.ppm");
    let image_size = Size2i::new(400, 225);
    let samples_per_pixel = 100;
    let max_depth = 50;

    let viewport_width = 4.0;
    let viewport_height = image_size.aspect_ratio() * viewport_width;
    let camera = Camera::new(
        viewport_width,
        viewport_height,
        1.0,
        Point3::ORIGIN,
        Dir3::UP,
        Dir3::FORWARD,
    );

    let world = {
        let mut world = HittableList::new();
        world.push(Sphere::new(Point3::ORIGIN + Dir3::FORWARD, 0.5));
        world.push(Sphere::new(Point3::ORIGIN + Dir3::DOWN * 100.5, 100.0));
        world
    };

    let distr = Uniform::new(
        Vec2f { x: 0.0, y: 0.0 },
        Vec2f {
            x: 1.0 / (image_size.width - 1) as f32,
            y: 1.0 / (image_size.height - 1) as f32,
        },
    );
    let mut rng = rand::thread_rng();

    let color_at_viewport = |pixel: Vec2f, rng: &mut ThreadRng| -> Color {
        ray_color(camera.ray(pixel), &world, rng, max_depth)
    };
    let pixels_iter = image_size
        .iterf()
        .map(|f| {
            (0..samples_per_pixel)
                .map(|_| color_at_viewport(f + rng.sample(&distr), &mut rng))
                .sum::<Color>()
        })
        .map(|c| c / samples_per_pixel as f32)
        .map(|c| c.gamma2());
    let pixels = collect_with_progress(pixels_iter, image_size.count());

    let out = OpenOptions::new().write(true).create(true).open(path)?;
    write_ppm_image(out, pixels, image_size)
}

fn collect_with_progress<I: IntoIterator>(iter: I, count: i32) -> Vec<I::Item> {
    let one_percent = (count / 100) as usize;
    let mut result = Vec::new();
    result.reserve(count as usize);
    for x in iter {
        result.push(x);
        if result.len() % one_percent == 0 {
            let percent = result.len() as f32 / one_percent as f32;
            eprintln!("{percent} %");
        }
    }
    eprintln!("Done!!!");
    result
}

fn write_ppm_image<I: IntoIterator<Item = Color>>(
    mut out: File,
    pixels: I,
    image_size: Size2i,
) -> Result<(), std::io::Error> {
    out.write_all(
        fmt::format(format_args!(
            "P3\n{} {}\n255\n",
            image_size.width, image_size.height
        ))
        .as_bytes(),
    )?;
    for pix in pixels {
        out.write_all(pix.to_ppm_string().as_bytes())?;
        out.write_all("\n".as_bytes())?;
    }

    Ok(())
}
