use rand::{Rng, SeedableRng};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, HittableList, MovingHittable, Sphere, self};

use crate::material::{Material, self};
use crate::texture::Texture;
use crate::vec2::Vec2f;
use crate::vec3::{Dir3, Point3};

pub struct World<T : Hittable>
{
    pub camera : Camera,
    pub hittable : T,
}

pub fn create_world_moving_spheres<'a>(aspect_ratio: f32, arena : &'a mut bumpalo::Bump) -> World<HittableList<Box<dyn 'a + Hittable>>> {
    // One large sphere as ground,
    // One sphere moving fast from left to right
    // One sphere moving fast fro up to down

    let viewport_width = 1.2;
    let viewport_height = aspect_ratio * viewport_width;
    let camera = Camera::new_look_at(
        viewport_width,
        viewport_height,
        Point3::new(0.0, 2.0, 10.0),
        Dir3::UP,
        Point3::new(0.0, 2.0, 0.0),
        0.0,
        0.0,
        0.0..=0.5,
        Vec2f::ZERO,
    );

    
    let tex_red = arena.alloc(Texture::Solid {color: Color::new_rgb(0.6, 0.2, 0.2)});
    let tex_blue = arena.alloc(Texture::Solid {color: Color::new_rgb(0.2, 0.2, 0.6)});
    let tex_black = arena.alloc(Texture::Solid {color: Color::new_rgb(0.0, 0.0, 0.0)});
    let tex_white = arena.alloc(Texture::Solid {color: Color::new_rgb(1.0, 1.0, 1.0)});
    let tex_ground = arena.alloc(Texture::Checker { frequency: 10.0, even: tex_black, odd: tex_white });
    let mat_ground = arena.alloc(Material::Lambert { albedo: tex_ground });
    let mat_red = arena.alloc(Material::Lambert { albedo: tex_red });
    let mat_blue = arena.alloc(Material::Lambert { albedo: tex_blue });

    let hittable = {
        let mut world = HittableList::<Box<dyn Hittable>>::new();
        let ground_radius = 100.0;
        let ground_center = Point3::new(0.0, -ground_radius, 0.0);
        world.push(Box::new(Sphere::new(ground_center,ground_radius,mat_ground)));

        let sphere1 = Sphere::new(Point3::new(-2.0, 1.5, 0.0), 0.5, mat_red);
        let moving_sphere_1 = MovingHittable::new(sphere1, Dir3::new(2.0, 0.0, 0.0));

        let sphere2 = Sphere::new(Point3::new(0.0, 0.5, 0.0), 0.5, mat_blue);
        let moving_sphere_2 = MovingHittable::new(sphere2, Dir3::new(0.0, 1.0, 0.0));

        world.push(Box::new(moving_sphere_1));
        world.push(Box::new(moving_sphere_2));

        world
    };

    World {
        camera,
        hittable,
    }
}

/*
pub fn create_world_random_scene(
    aspect_ratio: f32,
    seed: <rand_xoshiro::Xoroshiro128PlusPlus as SeedableRng>::Seed,
) -> (Camera, HittableList<Sphere>) {
    let viewport_width = 1.2;
    let viewport_height = aspect_ratio * viewport_width;
    let camera = Camera::new_look_at(
        viewport_width,
        viewport_height,
        Point3::new(13.0, 2.0, 3.0),
        Dir3::UP,
        Point3::ORIGIN,
        0.1,
        -3.5,
        0.0..=0.0,
        Vec2f::ZERO,
    );

    let mut rng = rand_xoshiro::Xoroshiro128PlusPlus::from_seed(seed);

    let world = {
        let mut world = HittableList::<Sphere>::new();
        let ground_material = Material::Lambert {
            albedo: Texture::Solid {
                color: Color::new_rgb(0.5, 0.5, 0.5),
            },
        };
        let ground_radius = 1000.0;
        let ground_center = Point3::new(0.0, -ground_radius, 0.0);
        world.push(Sphere::new(ground_center, ground_radius, ground_material));
        let rand_color = |rng: &mut rand_xoshiro::Xoroshiro128PlusPlus| -> Color {
            Color::new_rgb_arr(rng.gen::<[f32; 3]>())
        };
        for a in -11..=11 {
            for b in -11..=11 {
                let center = Point3::new(
                    a as f32 + 0.9 * rng.gen::<f32>(),
                    0.2,
                    b as f32 + 0.9 * rng.gen::<f32>(),
                );
                if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    let material_sample = rng.gen::<f32>();
                    let material = if material_sample < 0.8 {
                        let albedo = Color::convolution(rand_color(&mut rng), rand_color(&mut rng));
                        Material::Lambert {
                            albedo: Texture::Solid { color: albedo },
                        }
                    } else if material_sample < 0.95 {
                        let albedo = rand_color(&mut rng);
                        let fuzz = rng.gen_range(0.0..0.5);
                        Material::Metal {
                            albedo: Texture::Solid { color: albedo },
                            fuzz,
                        }
                    } else {
                        Material::Dielectric {
                            index_of_refraction: 1.5,
                        }
                    };

                    let small_radius = 0.2;
                    let real_center = ground_center
                        + (center - ground_center).with_length(ground_radius + small_radius);
                    world.push(Sphere::new(real_center, small_radius, material));
                }
            }
        }

        world.push(Sphere::new(
            Point3::new(0.0, 1.0, 0.0),
            1.0,
            Material::Dielectric {
                index_of_refraction: 1.5,
            },
        ));
        world.push(Sphere::new(
            Point3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::Lambert {
                albedo: Texture::Solid {
                    color: Color::new_rgb(0.4, 0.2, 0.1),
                },
            },
        ));
        world.push(Sphere::new(
            Point3::new(4.0, 1.0, 0.0),
            1.0,
            Material::Metal {
                albedo: Texture::Solid {
                    color: Color::new_rgb(0.7, 0.6, 0.5),
                },
                fuzz: 0.0,
            },
        ));
        world
    };

    (camera, world)
}


pub fn create_world_defocus_blur(aspect_ratio: f32) -> (Camera, HittableList<Sphere>) {
    let viewport_width = 1.2;
    let viewport_height = aspect_ratio * viewport_width;
    let camera = Camera::new_look_at(
        viewport_width,
        viewport_height,
        Point3::ORIGIN + Dir3::BACKWARD * 3.0 + Dir3::UP * 3.0 + 3.0 * Dir3::RIGHT,
        Dir3::UP,
        Point3::ORIGIN + Dir3::FORWARD,
        0.1,
        0.0,
        0.0..=0.0,
        Vec2f::ZERO,
    );
    let world = {
        let mut world = HittableList::<Sphere>::new();
        let material_ground = Material::Lambert {
            albedo: Texture::Solid {
                color: Color::new_rgb(0.8, 0.8, 0.0),
            },
        };
        let material_center = Material::Lambert {
            albedo: Texture::Solid {
                color: Color::new_rgb(0.7, 0.3, 0.3),
            },
        };
        let material_left = Material::Metal {
            albedo: Texture::Solid {
                color: Color::new_rgb(0.6, 0.6, 0.8),
            },
            fuzz: 0.05,
        };
        let material_right = Material::Metal {
            albedo: Texture::Solid {
                color: Color::new_rgb(0.8, 0.6, 0.2),
            },
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
 */