use rand::{SeedableRng, Rng};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HittableList, Sphere};

use crate::material::Material;
use crate::vec3::{Dir3, Point3};

pub fn create_world_random_scene(aspect_ratio: f32, seed: <rand_xoshiro::Xoroshiro128PlusPlus as SeedableRng>::Seed) -> (Camera, HittableList) {
    let viewport_width = 1.2;
    let viewport_height = aspect_ratio * viewport_width;
    let camera = Camera::new_look_at(
        viewport_width,
        viewport_height,
        Point3::new(13.0, 2.0, 3.0),
        Dir3::UP,
        Point3::ORIGIN,
        0.1,
        -3.5
    );

    let mut rng = rand_xoshiro::Xoroshiro128PlusPlus::from_seed(seed);

    let world = {
        let mut world = HittableList::new();
        let ground_material = Material::Lambert { albedo: Color::new_rgb(0.5, 0.5, 0.5) };
        let ground_radius = 1000.0;
        let ground_center = Point3::new(0.0, -ground_radius, 0.0);
        world.push(Sphere::new(ground_center, ground_radius, ground_material));
        let rand_color = |rng : &mut rand_xoshiro::Xoroshiro128PlusPlus| -> Color {Color::new_rgb_arr(rng.gen::<[f32;3]>())};
        for a in -11..=11 {
            for b in -11..=11 {
                let center = Point3::new(a as f32 + 0.9 * rng.gen::<f32>(), 0.2, b as f32 + 0.9 * rng.gen::<f32>());
                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let material_sample = rng.gen::<f32>();
                    let material = if material_sample < 0.8 {
                        let albedo = Color::convolution(rand_color(&mut rng), rand_color(&mut rng));
                        Material::Lambert { albedo: albedo }
                    } else if material_sample < 0.95 {
                        let albedo = rand_color(&mut rng);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Material::Metal { albedo: albedo, fuzz: fuzz }
                    } else {
                        Material::Dielectric { index_of_refraction: 1.5 }
                    };

                    let small_radius = 0.2;
                    let real_center = ground_center + (center - ground_center).with_length(ground_radius + small_radius);
                    world.push(Sphere::new(real_center, small_radius, material));
                }
            }
        }

        world.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Material::Dielectric { index_of_refraction: 1.5 }));
        world.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Material::Lambert { albedo: Color::new_rgb(0.4, 0.2,0.1) }));
        world.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Material::Metal { albedo: Color::new_rgb(0.7, 0.6, 0.5), fuzz: 0.0 }));
        world
    };

    (camera, world)
}

pub fn create_world_defocus_blur(aspect_ratio: f32) -> (Camera, HittableList) {
    let viewport_width = 1.2;
    let viewport_height = aspect_ratio * viewport_width;
    let camera = Camera::new_look_at(
        viewport_width,
        viewport_height,
        Point3::ORIGIN + Dir3::BACKWARD * 3.0 + Dir3::UP * 3.0 + 3.0*Dir3::RIGHT,
        Dir3::UP,
        Point3::ORIGIN + Dir3::FORWARD,
        0.1,
        0.0
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
    (camera, world)
}