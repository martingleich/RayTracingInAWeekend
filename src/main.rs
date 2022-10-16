mod camera;
mod color;
mod hittable;
mod math;
mod ray;
mod size2i;
mod vec2;
mod vec3;

use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use camera::Camera;
use color::Color;
use hittable::Hittable;
use hittable::HittableList;
use hittable::Sphere;

use rand::distributions::Uniform;
use rand::Rng;
use ray::Ray;
use size2i::Size2i;
use vec2::Vec2f;
use vec3::Dir3;
use vec3::Point3;

fn ray_color<THit: Hittable>(ray: Ray, world: &THit) -> Color {
    if let Some(interaction) = world.hit(ray, 0.0, f32::INFINITY) {
        let n = interaction.normal;
        return 0.5
            * Color::new_rgb(
                Dir3::dot(n, Dir3::RIGHT) + 1.0,
                Dir3::dot(n, Dir3::UP) + 1.0,
                Dir3::dot(n, Dir3::FORWARD) + 1.0,
            );
    }
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (Dir3::dot(Dir3::UP, unit_direction) + 1.0);
    let ground_color = Color::new_rgb(0.5, 0.7, 1.0);
    let sky_color = Color::new_rgb(1.0, 1.0, 1.0);

    return math::lerp(sky_color, ground_color, t);
}

fn main() -> Result<(), std::io::Error> {
    let path = Path::new("output/image.ppm");
    let image_size = Size2i::new(400, 225);
    let samples_per_pixel = 1;

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

    let mut world = HittableList::new();
    world.push(Sphere::new(Point3::ORIGIN + Dir3::FORWARD, 0.5));
    world.push(Sphere::new(Point3::ORIGIN + Dir3::DOWN * 1000.5, 1000.0));

    let distr = Uniform::new(
        Vec2f { x: 0.0, y: 0.0 },
        Vec2f {
            x: 1.0 / (image_size.width - 1) as f32,
            y: 1.0 / (image_size.height - 1) as f32,
        },
    );
    let mut rnd = rand::thread_rng();

    let mut out = OpenOptions::new().write(true).create(true).open(path)?;

    out.write_all(
        fmt::format(format_args!(
            "P3\n{} {}\n255\n",
            image_size.width, image_size.height
        ))
        .as_bytes(),
    )?;

    let color_at_viewport = |pixel: Vec2f| -> Color { ray_color(camera.ray(pixel), &world) };

    for f in image_size.iterf() {
        let pixel_color: Color = (0..samples_per_pixel)
            .map(|_| color_at_viewport(f + rnd.sample(&distr)))
            .sum::<Color>()
            / (samples_per_pixel as f32);

        out.write_all(pixel_color.to_ppm_string().as_bytes())?;
        out.write_all("\n".as_bytes())?;
    }
    out.flush()?;

    Ok(())
}
