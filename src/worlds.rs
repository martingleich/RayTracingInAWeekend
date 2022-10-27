use rand::{Rng, SeedableRng};

use crate::aabb::Aabb;
use crate::camera::Camera;
use crate::color::Color;
use crate::common;
use crate::hittable::{
    AxisAlignedBox, Hittable, HittableList, MovingHittable, Rect, Sphere, TransformedHittable, ConstantMedium, BoundingVolumeHierarchy,
};

use crate::background_color::BackgroundColor;
use crate::material::Material;
use crate::texture::Texture;
use crate::transformations::{RotationAroundUp, Translation};
use crate::vec3::{Dir3, Point3};

use self::create_utils::{solid_diffuse_light, smoke};
use crate::worlds::create_utils::solid_lambert;

pub struct World<T: Hittable> {
    pub camera: Camera,
    pub hittable: T,
    pub background: BackgroundColor,
}

mod create_utils {
    use crate::{color::Color, material::Material, texture::Texture};

    pub fn solid_lambert(arena: &bumpalo::Bump, color: Color) -> &Material {
        let texture = arena.alloc(Texture::Solid { color });
        arena.alloc(Material::Lambert { albedo: texture })
    }

    pub fn solid_diffuse_light(arena: &bumpalo::Bump, color: Color) -> &Material {
        let texture = arena.alloc(Texture::Solid { color });
        arena.alloc(Material::DiffuseLight { emit: texture })
    }

    pub fn smoke(arena: &bumpalo::Bump, color: Color) -> &Material {
        let texture = arena.alloc(Texture::Solid { color });
        arena.alloc(Material::Isotropic {  albedo: texture })
    }
}

pub fn create_world_final_scene2<'a>(
    arena: &'a mut bumpalo::Bump,
    seed: <common::TRng as SeedableRng>::Seed,
) -> World<BoundingVolumeHierarchy<AxisAlignedBox>> {
    let mut rng = common::TRng::from_seed(seed);

    let camera = Camera::build()
        .vertical_fov(40.0, 1.0)
        .position(Point3::new(478.0, 278.0, -600.0))
        .look_at(Dir3::UP, Point3::new(278.0, 278.0, 0.0))
        .build();

    let ground = solid_lambert(arena, Color::new_rgb(0.48, 0.83, 0.53));
    let boxes = {
        let mut boxes = Vec::new();
        let boxes_per_side = 40;
        let range = Aabb::new_corners(Point3::new(-1000.0, 0.0, -1000.0), Point3::new(1000.0, 101.0, 1000.0));
        let width = (range.max.0.e[0] - range.min.0.e[0]) / boxes_per_side as f32;
        let height = range.max.0.e[1] - range.min.0.e[1];
        let depth = (range.max.0.e[2] - range.min.0.e[2]) / boxes_per_side as f32;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let min = range.min + Dir3::new(i as f32 * width, 0.0, j as f32 * depth);
                let max = min + Dir3::new(width, height * rng.gen::<f32>(), depth);
                let abox = AxisAlignedBox::new(&Aabb::new_corners(min, max), ground);
                boxes.push(abox);
            }
        }
        boxes
    };
    let bvh = BoundingVolumeHierarchy::new(boxes, &camera.time_interval);

    let hittable = bvh;

    World {
        background: BackgroundColor::Sky,
        camera,
        hittable,
    }
}

pub fn create_world_cornell_box_smoke<'a>(
    arena: &'a mut bumpalo::Bump,
    _seed: <common::TRng as SeedableRng>::Seed,
) -> World<Vec<Box<dyn 'a + Hittable>>> {
    let hsize = 278.0;
    let camera = Camera::build()
        .vertical_fov(40.0, 1.0)
        .position(Point3::new(hsize, hsize, -800.0))
        .look_at(Dir3::UP, Point3::new(hsize, hsize, 0.0))
        .build();

    let red = solid_lambert(arena, Color::new_rgb(0.65, 0.05, 0.05));
    let white = solid_lambert(arena, Color::new_rgb(0.73, 0.73, 0.73));
    let green = solid_lambert(arena, Color::new_rgb(0.12, 0.45, 0.15));
    let smoke_black = smoke(arena, Color::new_rgb(0.0, 0.0, 0.0));
    let smoke_white = smoke(arena, Color::new_rgb(1.0, 1.0, 1.0));
    let light = solid_diffuse_light(arena, Color::new_rgb(7.0, 7.0, 7.0));

    let hittable = {
        let walls = {
            let mut items = Vec::new();
            items.push(Rect::new_yz(
                Point3::new(0.0, hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                red,
            ));
            items.push(Rect::new_yz(
                Point3::new(2.0 * hsize, hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                green,
            ));
            items.push(Rect::new_xz(
                Point3::new(hsize, 0.0, hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ));
            items.push(Rect::new_xz(
                Point3::new(hsize, 2.0 * hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ));
            items.push(Rect::new_xy(
                Point3::new(hsize, hsize, 2.0 * hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ));
            items.push(Rect::new_xz(
                Point3::new(hsize, 2.0 * hsize - 1.0, hsize),
                300.0,
                300.0,
                light,
            ));
            items
        };

        let boxes = {
            // let mut boxes = HittableList::new();
            // let sphere = arena.alloc(Sphere::new(Point3::new(265.0, 150.0, 295.0), 100.0, smoke_black));
            // boxes.push(ConstantMedium::new(sphere, smoke_black, 0.01));
            let mut boxes = Vec::new();
            let box1_base = arena.alloc(AxisAlignedBox::new(
                &Aabb {
                    min: Point3::new(0.0, 0.0, 0.0),
                    max: Point3::new(165.0, 330.0, 165.0),
                },
                white,
            ));
            let box1_rot = arena.alloc(TransformedHittable {
                hittable: box1_base,
                transformation: RotationAroundUp::new(-15.0),
            });
            let box1 = arena.alloc(TransformedHittable {
                hittable: box1_rot,
                transformation: Translation {
                    offset: Dir3::new(265.0, 0.0, 295.0),
                },
            });
            boxes.push(ConstantMedium::new(box1, smoke_white, 0.01));

            let box2_base = arena.alloc(AxisAlignedBox::new(
                &Aabb {
                    min: Point3::new(0.0, 0.0, 0.0),
                    max: Point3::new(165.0, 165.0, 165.0),
                },
                white,
            ));
            let box2_rot = arena.alloc(TransformedHittable {
                hittable: box2_base,
                transformation: RotationAroundUp::new(18.0),
            });
            let box2 = arena.alloc(TransformedHittable {
                hittable: box2_rot,
                transformation: Translation {
                    offset: Dir3::new(130.0, 0.0, 65.0),
                },
            });
            boxes.push(ConstantMedium::new(box2, smoke_black, 0.01));

            boxes
        };

        let mut world = Vec::<Box<dyn Hittable>>::new();
        world.push(Box::new(walls));
        world.push(Box::new(boxes));

        world
    };

    World {
        background: BackgroundColor::Solid {
            color: Color::BLACK,
        },
        camera,
        hittable,
    }
}

pub fn create_world_cornell_box<'a>(
    arena: &'a mut bumpalo::Bump,
    _seed: <common::TRng as SeedableRng>::Seed,
) -> World<Vec<Box<dyn 'a + Hittable>>> {
    let hsize = 278.0;
    let camera = Camera::build()
        .vertical_fov(40.0, 1.0)
        .position(Point3::new(hsize, hsize, -800.0))
        .look_at(Dir3::UP, Point3::new(hsize, hsize, 0.0))
        .build();

    let red = solid_lambert(arena, Color::new_rgb(0.65, 0.05, 0.05));
    let white = solid_lambert(arena, Color::new_rgb(0.73, 0.73, 0.73));
    let green = solid_lambert(arena, Color::new_rgb(0.12, 0.45, 0.15));
    let light = solid_diffuse_light(arena, Color::new_rgb(15.0, 15.0, 15.0));

    let hittable = {
        let walls = {
            let mut items = Vec::new();
            items.push(Rect::new_yz(
                Point3::new(0.0, hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                red,
            ));
            items.push(Rect::new_yz(
                Point3::new(2.0 * hsize, hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                green,
            ));
            items.push(Rect::new_xz(
                Point3::new(hsize, 0.0, hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ));
            items.push(Rect::new_xz(
                Point3::new(hsize, 2.0 * hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ));
            items.push(Rect::new_xy(
                Point3::new(hsize, hsize, 2.0 * hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ));
            items.push(Rect::new_xz(
                Point3::new(hsize, 2.0 * hsize - 1.0, hsize),
                130.0,
                130.0,
                light,
            ));
            items
        };

        let boxes = {
            let mut boxes = Vec::new();
            let box1_base = arena.alloc(AxisAlignedBox::new(
                &Aabb {
                    min: Point3::new(0.0, 0.0, 0.0),
                    max: Point3::new(165.0, 330.0, 165.0),
                },
                white,
            ));
            let box1_rot = arena.alloc(TransformedHittable {
                hittable: box1_base,
                transformation: RotationAroundUp::new(-15.0),
            });
            let box1 = TransformedHittable {
                hittable: box1_rot,
                transformation: Translation {
                    offset: Dir3::new(265.0, 0.0, 295.0),
                },
            };
            boxes.push(box1);

            let box2_base = arena.alloc(AxisAlignedBox::new(
                &Aabb {
                    min: Point3::new(0.0, 0.0, 0.0),
                    max: Point3::new(165.0, 165.0, 165.0),
                },
                white,
            ));
            let box2_rot = arena.alloc(TransformedHittable {
                hittable: box2_base,
                transformation: RotationAroundUp::new(18.0),
            });
            let box2 = TransformedHittable {
                hittable: box2_rot,
                transformation: Translation {
                    offset: Dir3::new(130.0, 0.0, 65.0),
                },
            };
            boxes.push(box2);

            boxes
        };

        let mut world = Vec::<Box<dyn Hittable>>::new();
        world.push(Box::new(walls));
        world.push(Box::new(boxes));

        world
    };

    World {
        background: BackgroundColor::Solid {
            color: Color::BLACK,
        },
        camera,
        hittable,
    }
}

pub fn create_world_simple_plane(
    arena: &mut bumpalo::Bump,
    _seed: <common::TRng as SeedableRng>::Seed,
) -> World<Vec<Rect>> {
    // A single rectangle with a solid
    let camera = Camera::build()
        .vertical_fov(60.0, 16.0/9.0)
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
        let mut list = Vec::new();
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

pub fn create_world_earth_mapped(
    arena: &mut bumpalo::Bump,
    _seed: <common::TRng as SeedableRng>::Seed,
) -> World<Sphere> {
    // A single sphere with a image texture
    let camera = Camera::build()
        .vertical_fov(60.0, 16.0/19.0)
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
    arena: &'a mut bumpalo::Bump,
    _seed: <common::TRng as SeedableRng>::Seed,
) -> World<Vec<Box<dyn 'a + Hittable>>> {
    // One large sphere as ground,
    // One sphere moving fast from left to right
    // One sphere moving fast fro up to down

    let camera = Camera::build()
        .vertical_fov(60.0, 16.0/19.0)
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
        inv_frequency: 10.0,
        even: tex_black,
        odd: tex_white,
    });
    let mat_ground = arena.alloc(Material::Lambert { albedo: tex_ground });
    let mat_red = arena.alloc(Material::Lambert { albedo: tex_red });
    let mat_blue = arena.alloc(Material::Lambert { albedo: tex_blue });

    let hittable = {
        let mut world = Vec::<Box<dyn Hittable>>::new();
        let ground_radius = 100.0;
        let ground_center = Point3::new(0.0, -ground_radius, 0.0);
        world.push(Box::new(Sphere::new(
            ground_center,
            ground_radius,
            mat_ground,
        )));

        let sphere1 = arena.alloc(Sphere::new(Point3::new(-2.0, 1.5, 0.0), 0.5, mat_red));
        let moving_sphere_1 = MovingHittable::new(sphere1, Dir3::new(2.0, 0.0, 0.0));

        let sphere2 = arena.alloc(Sphere::new(Point3::new(0.0, 0.5, 0.0), 0.5, mat_blue));
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
    arena: &mut bumpalo::Bump,
    seed: <common::TRng as SeedableRng>::Seed,
) -> World<Vec<Sphere>> {
    let camera = Camera::build()
        .vertical_fov(60.0, 16.0/9.0)
        .position(Point3::new(13.0, 2.0, 3.0))
        .look_at(Dir3::UP, Point3::ORIGIN)
        .focus_distance(10.0)
        .aperture(0.1)
        .build();

    let mut rng = common::TRng::from_seed(seed);

    let hittable = {
        let mut world = Vec::<Sphere>::new();
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
    arena: &mut bumpalo::Bump,
    _seed: <common::TRng as SeedableRng>::Seed,
) -> World<Vec<Sphere>> {
    let camera = Camera::build()
        .vertical_fov(60.0, 16.0/9.0)
        .position(Point3::ORIGIN + Dir3::BACKWARD * 3.0 + Dir3::UP * 3.0 + 3.0 * Dir3::RIGHT)
        .look_at_focus(Dir3::UP, Point3::ORIGIN + Dir3::FORWARD)
        .aperture(0.1)
        .build();
    let hittable = {
        let mut world = Vec::<Sphere>::new();
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
