use rand::{distributions::Uniform, Rng, SeedableRng};
use std::{sync::mpsc, thread};

use crate::{
    background_color::BackgroundColor,
    color::Color,
    common::{self, TRng},
    Camera, Dir3, MaterialScatteringDistribution, Ray, Scene, Size2i, Vec2f,
    WorldScatteringDistribution, WorldScatteringDistributionProvider,
};

pub struct World<'a> {
    pub camera: Camera,
    pub hittable: &'a Scene<'a>,
    pub background: BackgroundColor,
    pub scattering_distribution_provider: Option<WorldScatteringDistributionProvider>,
}

fn ray_color<'a>(ray: &Ray, world: &World<'a>, rng: &mut TRng, max_depth: i32) -> Color {
    let mut depth = max_depth;
    let mut accum_attentuation: Color = Color::WHITE;
    let mut accum_emitted: Color = Color::BLACK;
    let mut cur_ray = *ray;
    loop {
        if let Some(interaction) = world.hittable.hit(&cur_ray, &(0.001..f32::INFINITY), rng) {
            if depth <= 1 {
                return Color::BLACK;
            } else if let Some((attentuation, material_scattering_distribution)) =
                interaction.material.scatter(&cur_ray, &interaction, rng)
            {
                let (scattered, probablity) = if material_scattering_distribution.is_discrete() {
                    let scattered_dir = material_scattering_distribution.generate(rng);
                    let scattered = Ray::new(interaction.position, scattered_dir, cur_ray.time);
                    (scattered, 1.0)
                } else {
                    let world_scattering_distribution = world
                        .scattering_distribution_provider
                        .as_ref()
                        .and_then(|p| p.generate(&interaction.position));
                    let (scattered_pdf, scattered_dir) = sample_final_scattering_distribution(
                        &world_scattering_distribution,
                        &material_scattering_distribution,
                        rng,
                    );
                    let scattered = Ray::new(interaction.position, scattered_dir, cur_ray.time);
                    let scattering_pdf =
                        interaction
                            .material
                            .scattering_pdf(&cur_ray, &scattered, &interaction);
                    let probablity = scattering_pdf / scattered_pdf;
                    (scattered, probablity)
                };

                let emitted = interaction.material.emit(&interaction);

                accum_emitted += Color::convolution(accum_attentuation, emitted);
                accum_attentuation =
                    Color::convolution(accum_attentuation, attentuation) * probablity;
                cur_ray = scattered;
                depth -= 1;
                continue;
            } else {
                let emitted = interaction.material.emit(&interaction);
                return accum_emitted + Color::convolution(accum_attentuation, emitted);
            }
        } else {
            let emitted = world.background.sample(ray);
            return accum_emitted + Color::convolution(accum_attentuation, emitted);
        }
    }
}

fn sample_final_scattering_distribution(
    world: &Option<WorldScatteringDistribution>,
    material: &MaterialScatteringDistribution,
    rng: &mut common::TRng,
) -> (f32, Dir3) {
    if let Some(world) = world {
        let mix: f32 = 0.5;
        let direction = if rng.gen_bool(mix as f64) {
            world.generate(rng)
        } else {
            material.generate(rng)
        };
        let p = mix * world.value(direction) + (1.0 - mix) * material.value(direction);
        (p, direction)
    } else {
        let direction = material.generate(rng);
        let p = material.value(direction);
        (p, direction)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RenderMode {
    Default,
    Normals,
}

impl RenderMode {
    fn ray_color<'a>(
        self,
        ray: &Ray,
        world: &'a World<'a>,
        rng: &mut TRng,
        max_depth: i32,
    ) -> Color {
        match self {
            RenderMode::Default => ray_color(ray, world, rng, max_depth),
            RenderMode::Normals => {
                if let Some(interaction) = world.hittable.hit(ray, &(0.001..f32::INFINITY), rng) {
                    Color((interaction.normal.0 + crate::Vec3::new(1.0, 1.0, 1.0)) * 0.5)
                } else {
                    world.background.sample(ray)
                }
            }
        }
    }
}

pub fn render(
    image_size: Size2i,
    thread_count: usize,
    samples_per_pixel: usize,
    max_depth: i32,
    world: &World,
    render_mode: RenderMode,
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
                    let mut sub_rng = TRng::from_rng(&mut rng).unwrap();
                    move |fpix: Vec2f| {
                        (0..real_samples_per_pixel)
                            .map(|_| {
                                let pix = fpix + sub_rng.sample(pixel_sample_distr_ref);
                                let ray = world.camera.ray(&mut sub_rng, pix);
                                render_mode.ray_color(&ray, world, &mut sub_rng, max_depth)
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
        for (i, p) in plane.iter().enumerate() {
            pixels[i] += *p;
        }
    }
    for pix in &mut pixels {
        *pix *= multiplier;
    }
    pixels
}
