mod camera;
mod color;
mod hittable;
mod material;
mod math;
mod ray;
mod size2i;
mod vec2;
mod vec3;

use std::{path::Path, sync::mpsc, thread};

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
    max_depth: i32,
) -> Color {
    let mut depth = max_depth;
    let mut attentuation: Color = Color::WHITE;
    let mut cur_ray = *ray;
    loop {
        if let Some(interaction) = world.hit(&cur_ray, &(0.0001..f32::INFINITY)) {
            if depth <= 1 {
                return Color::BLACK;
            } else if let Some((attentuation2, scattered)) =
                interaction.material.scatter(&cur_ray, &interaction, rng)
            {
                attentuation = Color::convolution(attentuation, attentuation2);
                cur_ray = scattered;
                depth -= 1;
                continue;
            } else {
                return Color::BLACK;
            }
        } else {
            return Color::convolution(attentuation, sky_color(ray));
        }
    }
}

fn main() -> Result<(), ImageError> {
    let path = Path::new("output/image.png");
    let image_size = Size2i::new(800, 450);
    let samples_per_pixel = 200;
    let max_depth = 50;
    let thread_count = thread::available_parallelism().map_or(4, |x| x.get());
    eprintln!("Using {thread_count} threads.");

    let viewport_width = 1.2;
    let viewport_height = image_size.aspect_ratio() * viewport_width;

    let camera = Camera::new_look_at(
        viewport_width,
        viewport_height,
        1.0,
        Point3::ORIGIN + Dir3::BACKWARD * 3.0 + Dir3::UP * 3.0 + Dir3::RIGHT,
        Dir3::UP,
        Point3::ORIGIN,
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
            fuzz: 0.05,
        };
        let material_right = Material::Metal {
            albedo: Color::new_rgb(0.8, 0.6, 0.2),
            fuzz: 0.5,
        };
        let material_front = Material::Dielectric {
            index_of_refraction: 1.5,
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
        world.push(Sphere::new(
            Point3::ORIGIN + 0.5 * Dir3::LEFT + 0.3 * Dir3::UP,
            0.3,
            material_front,
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
    let start_time = std::time::Instant::now();

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
    let mut last_output = usize::MAX;
    let report_update_thread = thread::spawn(move || loop {
        match rx.recv() {
            Ok((id, done)) => {
                done_work[id] = done;
                let total_done = done_work.iter().sum::<usize>();
                let done_percent = (100 * total_done) / total_work;
                if last_output != done_percent {
                    last_output = done_percent;
                    eprint!("\r{done_percent} %");
                }
            }
            Err(_) => return,
        };
    });

    let pixel_sample_distr_ref = &pixel_sample_distr;
    let mut rng = rand::rngs::StdRng::from_entropy();
    let planes = thread::scope(|s| {
        let work_tasks = split_work_tasks(samples_per_pixel, thread_count);

        #[allow(clippy::needless_collect)]
        // reason:"First start all threads, the collect them. Removing the collect would serialize the threads")]
        let threads = work_tasks
            .into_iter()
            .map(|(thread_id, real_samples_per_pixel)| {
                let render_pixel = {
                    let mut sub_rng =
                        rand_xoshiro::Xoroshiro128PlusPlus::from_rng(&mut rng).unwrap();
                    move |fpix: Vec2f| {
                        (0..real_samples_per_pixel)
                            .map(|_| {
                                let pix = fpix + sub_rng.sample(pixel_sample_distr_ref);
                                let ray = camera.ray(pix);
                                ray_color(&ray, world, &mut sub_rng, max_depth)
                            })
                            .sum::<Color>()
                            / real_samples_per_pixel as f32
                    }
                };

                let update_progress = {
                    let local_tx = tx.clone();
                    let mut count = 0;
                    move |fpix: Color| {
                        if count % (100 * real_samples_per_pixel) == 0 {
                            local_tx.send((thread_id, count)).unwrap();
                        }
                        count += real_samples_per_pixel;
                        fpix
                    }
                };

                let thread = move || {
                    image_size
                        .iterf()
                        .map(render_pixel)
                        .map(update_progress) // Sideeffect
                        .collect::<Vec<_>>()
                };
                s.spawn(thread)
            })
            .collect::<Vec<_>>();

        threads
            .into_iter()
            .map(|t| t.join().unwrap())
            .collect::<Vec<_>>()
    });
    drop(tx); // Drop the final sender, so the report_update_thread stops

    report_update_thread.join().unwrap();
    let passed_time = std::time::Instant::now() - start_time;
    let passed_time_seconds = passed_time.as_secs_f64();
    eprintln!("\rRendering done in {passed_time_seconds} seconds"); // Leading \r to remove progress bar

    eprintln!("Merging threads...");
    merge_planes(planes)
}

fn split_work_tasks(samples_per_pixel: usize, thread_count: usize) -> Vec<(usize, usize)> {
    let whole_sample_per_pixel = samples_per_pixel / thread_count;
    let remaining_samples_per_pixel = samples_per_pixel % thread_count;
    let mut work_tasks = Vec::new();
    let mut id = 0;
    while id < thread_count {
        let real_samples_per_pixel =
            whole_sample_per_pixel + (id < remaining_samples_per_pixel) as usize;
        if real_samples_per_pixel == 0 {
            break;
        }
        work_tasks.push((id, real_samples_per_pixel));
        id += 1;
    }
    work_tasks
}

fn merge_planes(mut planes: Vec<Vec<Color>>) -> Vec<Color> {
    let multiplier = 1.0 / planes.len() as f32;
    let mut pixels = planes.pop().unwrap();
    for plane in &planes {
        debug_assert_eq!(pixels.len(), plane.len());
        for (p, i) in plane.iter().enumerate() {
            pixels[p] += *i;
        }
    }
    for pix in &mut pixels {
        *pix *= multiplier;
    }
    pixels
}
