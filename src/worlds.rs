use rand::{Rng, SeedableRng};

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, HittableList, MovingHittable, Rect, Sphere};

use crate::background_color::BackgroundColor;
use crate::material::Material;
use crate::texture::Texture;
use crate::vec3::{Dir3, Point3};

pub struct World<T: Hittable> {
    pub camera: Camera,
    pub hittable: T,
    pub background: BackgroundColor,
}

pub fn create_world_simple_plane(
    aspect_ratio: f32,
    arena: &mut bumpalo::Bump,
) -> World<HittableList<Rect>> {
    // A single rectangle with a solid
    let camera = Camera::build()
        .vertical_fov(60.0, aspect_ratio)
        .position(Point3::new(0.0, 6.0, 10.0))
        .look_at(Dir3::UP, Point3::ORIGIN)
        .build();

    let tex_white = arena.alloc(Texture::Solid {
        color: 100.0 * Color::new_rgb(1.0, 1.0, 1.0),
    });
    let tex_blue = arena.alloc(Texture::Solid {
        color: Color::new_rgb(0.0, 0.0, 0.4),
    });
    let mat_emit = arena.alloc(Material::DiffuseLight { emit: tex_white });
    let mat_floor = arena.alloc(Material::Lambert { albedo: tex_blue });

    let hittable = {
        let mut list = HittableList::new();
        list.push(Rect::new_xy(Point3::new(0.0, 2.0, 0.0), 1.0, 1.0, mat_emit));

        list.push(Rect::new_xz(Point3::ORIGIN, 10.0, 10.0, mat_floor));

        list
    };

    World {
        camera,
        hittable,
        background: BackgroundColor::Solid {
            color: Color::new_rgb(0.1, 0.1, 0.1),
        },
    }
}

pub fn create_world_earth_mapped(aspect_ratio: f32, arena: &mut bumpalo::Bump) -> World<Sphere> {
    // A single sphere with a image texture
    let camera = Camera::build()
        .vertical_fov(60.0, aspect_ratio)
        .position(Point3::new(0.0, 2.0, 10.0))
        .look_at(Dir3::UP, Point3::ORIGIN)
        .build();

    let path = std::path::Path::new("input/earthmap.jpg");
    let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
    let reader = std::io::BufReader::new(file);
    let image = arena.alloc(image::load(reader, image::ImageFormat::Jpeg).unwrap());

    let tex_earth = arena.alloc(Texture::Image {
        image: image.as_rgb8().unwrap(),
    });
    let mat_earth = arena.alloc(Material::Lambert { albedo: tex_earth });

    let hittable = Sphere::new(Point3::ORIGIN, 2.0, mat_earth);

    World {
        camera,
        hittable,
        background: BackgroundColor::Sky,
    }
}

pub fn create_world_moving_spheres<'a>(
    aspect_ratio: f32,
    arena: &'a mut bumpalo::Bump,
) -> World<HittableList<Box<dyn 'a + Hittable>>> {
    // One large sphere as ground,
    // One sphere moving fast from left to right
    // One sphere moving fast fro up to down

    let camera = Camera::build()
        .vertical_fov(60.0, aspect_ratio)
        .position(Point3::new(0.0, 2.0, 10.0))
        .look_at(Dir3::UP, Point3::new(0.0, 2.0, 0.0))
        .motion_blur(0.0, 0.5)
        .build();

    let tex_red = arena.alloc(Texture::Solid {
        color: Color::new_rgb(0.6, 0.2, 0.2),
    });
    let tex_blue = arena.alloc(Texture::Solid {
        color: Color::new_rgb(0.2, 0.2, 0.6),
    });
    let tex_black = arena.alloc(Texture::Solid {
        color: Color::new_rgb(0.0, 0.0, 0.0),
    });
    let tex_white = arena.alloc(Texture::Solid {
        color: Color::new_rgb(1.0, 1.0, 1.0),
    });
    let tex_ground = arena.alloc(Texture::Checker {
        frequency: 10.0,
        even: tex_black,
        odd: tex_white,
    });
    let mat_ground = arena.alloc(Material::Lambert { albedo: tex_ground });
    let mat_red = arena.alloc(Material::Lambert { albedo: tex_red });
    let mat_blue = arena.alloc(Material::Lambert { albedo: tex_blue });

    let hittable = {
        let mut world = HittableList::<Box<dyn Hittable>>::new();
        let ground_radius = 100.0;
        let ground_center = Point3::new(0.0, -ground_radius, 0.0);
        world.push(Box::new(Sphere::new(
            ground_center,
            ground_radius,
            mat_ground,
        )));

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
        background: BackgroundColor::Sky,
    }
}

pub fn create_world_random_scene(
    aspect_ratio: f32,
    arena: &mut bumpalo::Bump,
    seed: <rand_xoshiro::Xoroshiro128PlusPlus as SeedableRng>::Seed,
) -> World<HittableList<Sphere>> {
    let camera = Camera::build()
        .vertical_fov(60.0, aspect_ratio)
        .position(Point3::new(13.0, 2.0, 3.0))
        .look_at(Dir3::UP, Point3::ORIGIN)
        .focus_distance(10.0)
        .aperture(0.1)
        .build();

    let mut rng = rand_xoshiro::Xoroshiro128PlusPlus::from_seed(seed);

    let hittable = {
        let mut world = HittableList::<Sphere>::new();
        let ground_tex = arena.alloc(Texture::Solid {
            color: Color::new_rgb(0.5, 0.5, 0.5),
        });
        let ground_material = arena.alloc(Material::Lambert { albedo: ground_tex });
        let ground_radius = 1000.0;
        let ground_center = Point3::new(0.0, -ground_radius, 0.0);
        world.push(Sphere::new(ground_center, ground_radius, ground_material));
        let rand_color = |rng: &mut rand_xoshiro::Xoroshiro128PlusPlus| -> Color {
            Color::new_rgb_arr(rng.gen::<[f32; 3]>())
        };
        let material_glass = &*arena.alloc(Material::Dielectric {
            index_of_refraction: 1.5,
        });
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
                        let tex = arena.alloc(Texture::Solid { color: albedo });
                        arena.alloc(Material::Lambert { albedo: tex })
                    } else if material_sample < 0.95 {
                        let albedo = rand_color(&mut rng);
                        let fuzz = rng.gen_range(0.0..0.5);
                        let tex = arena.alloc(Texture::Solid { color: albedo });
                        arena.alloc(Material::Metal { albedo: tex, fuzz })
                    } else {
                        material_glass
                    };

                    let small_radius = 0.2;
                    let real_center = ground_center
                        + (center - ground_center).with_length(ground_radius + small_radius);
                    world.push(Sphere::new(real_center, small_radius, material));
                }
            }
        }

        world.push(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_glass));
        let tex = arena.alloc(Texture::Solid {
            color: Color::new_rgb(0.4, 0.2, 0.1),
        });
        let material = arena.alloc(Material::Lambert { albedo: tex });
        world.push(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material));
        let tex = arena.alloc(Texture::Solid {
            color: Color::new_rgb(0.7, 0.6, 0.5),
        });
        let material = arena.alloc(Material::Metal {
            albedo: tex,
            fuzz: 0.0,
        });
        world.push(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material));
        world
    };

    World {
        camera,
        hittable,
        background: BackgroundColor::Sky,
    }
}

pub fn create_world_defocus_blur(
    aspect_ratio: f32,
    arena: &mut bumpalo::Bump,
) -> World<HittableList<Sphere>> {
    let camera = Camera::build()
        .vertical_fov(60.0, aspect_ratio)
        .position(Point3::ORIGIN + Dir3::BACKWARD * 3.0 + Dir3::UP * 3.0 + 3.0 * Dir3::RIGHT)
        .look_at_focus(Dir3::UP, Point3::ORIGIN + Dir3::FORWARD)
        .aperture(0.1)
        .build();
    let hittable = {
        let mut world = HittableList::<Sphere>::new();
        let tex = arena.alloc(Texture::Solid {
            color: Color::new_rgb(0.8, 0.8, 0.0),
        });
        let material_ground = arena.alloc(Material::Lambert { albedo: tex });
        let tex = arena.alloc(Texture::Solid {
            color: Color::new_rgb(0.7, 0.3, 0.3),
        });
        let material_center = arena.alloc(Material::Lambert { albedo: tex });
        let tex = arena.alloc(Texture::Solid {
            color: Color::new_rgb(0.6, 0.6, 0.8),
        });
        let material_left = arena.alloc(Material::Metal {
            albedo: tex,
            fuzz: 0.05,
        });
        let tex = arena.alloc(Texture::Solid {
            color: Color::new_rgb(0.8, 0.6, 0.2),
        });
        let material_right = arena.alloc(Material::Metal {
            albedo: tex,
            fuzz: 0.5,
        });
        let material_front = arena.alloc(Material::Dielectric {
            index_of_refraction: 1.5,
        });
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

    World {
        camera,
        hittable,
        background: BackgroundColor::Sky,
    }
}
