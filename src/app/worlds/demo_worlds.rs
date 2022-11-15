use rand::Rng;
use ray_tracing_in_a_weekend::*;

use super::world_builder::WorldBuilder;

pub fn create_world_suzanne<'a>(wb: &'a WorldBuilder<'a>, _rng: &'a mut common::TRng) -> World<'a> {
    let camera = Camera::build()
        .vertical_fov(40.0, 3.0 / 4.0)
        .position(Point3::new(0.0, 2.0, 10.0))
        .look_at(Dir3::UP, Point3::new(0.0, 0.0, 0.0))
        .build();
    let background = BackgroundColor::Sky;
    let mat_ground = wb.material_lambert_solid(Color::new_rgb(0.4, 0.4, 0.4));
    let mat_monkey = wb.material_lambert_solid(Color::new_rgb(1.0, 0.2, 0.2));
    let scene = wb
        .new_group()
        .add(wb.new_mesh_from_file_obj_uniform_material(
            std::path::Path::new("input/suzanne.obj"),
            mat_monkey,
        ))
        .add(
            wb.new_obj_sphere(1000.1, mat_ground)
                .translate(Dir3::new(0.0, -1000.0, 0.0)),
        )
        .build();

    scene.finish(wb, background, camera)
}

/*
pub fn create_world_final_scene2<'a>(
    arena: &'a mut bumpalo::Bump,
    rng: &mut common::TRng,
) -> World<impl Hittable + 'a> {
    let camera = Camera::build()
        .vertical_fov(40.0, 1.0)
        .position(Point3::new(478.0, 278.0, -600.0))
        .look_at(Dir3::UP, Point3::new(278.0, 278.0, 0.0))
        .motion_blur(0.0, 1.0)
        .build();

    let material_ground = solid_lambert(arena, Color::new_rgb(0.48, 0.83, 0.53));
    let boxes = {
        let mut boxes = Vec::new();
        let boxes_per_side = 20;
        let range = Aabb::new_corners(
            Point3::new(-1000.0, 0.0, -1000.0),
            Point3::new(1000.0, 101.0, 1000.0),
        );
        let width = (range.max.0.e[0] - range.min.0.e[0]) / boxes_per_side as f32;
        let height = range.max.0.e[1] - range.min.0.e[1];
        let depth = (range.max.0.e[2] - range.min.0.e[2]) / boxes_per_side as f32;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let min = range.min + Dir3::new(i as f32 * width, 0.0, j as f32 * depth);
                let max = min + Dir3::new(width, height * rng.gen::<f32>(), depth);
                let abox = AxisAlignedBox::new(&Aabb::new_corners(min, max), material_ground);
                boxes.push(abox);
            }
        }
        boxes
    };
    let boxes = BoundingVolumeHierarchy::new(boxes, &camera.time_interval);

    let material_light = solid_diffuse_light(arena, Color::new_rgb(7.0, 7.0, 7.0));
    let light = Rect::new_xz(
        Point3::new(273.0, 554.0, 279.0),
        300.0,
        300.0,
        material_light,
    );

    let material_glass = glass(arena, 1.5);
    let material_metal = solid_metal(arena, Color::new_rgb(0.8, 0.8, 0.9), 1.0);
    let texture_marble = arena.alloc(Texture::Marble {
        scale: 0.1,
        noise: Perlin::new(8, rng),
    });
    let material_marble = arena.alloc(Material::Lambert {
        albedo: texture_marble,
    });
    let texture_earth = {
        let path = std::path::Path::new("input/earthmap.jpg");
        let file = std::fs::OpenOptions::new().read(true).open(path).unwrap();
        let reader = std::io::BufReader::new(file);
        let image = arena.alloc(image::load(reader, image::ImageFormat::Jpeg).unwrap());
        arena.alloc(Texture::Image {
            image: image.as_rgb8().unwrap(),
        })
    };
    let material_earth = arena.alloc(Material::Lambert {
        albedo: texture_earth,
    });
    let obj_fog_glass = Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, material_glass);
    let obj_fog_glass2 = arena.alloc(obj_fog_glass.clone());
    let material_fog = smoke(arena, Color::new_rgb(0.2, 0.4, 0.9));
    let obj_fog_sphere = ConstantMedium::new(obj_fog_glass2, material_fog, 0.2);

    let spheres: Vec<Sphere> = vec![
        Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, material_glass),
        obj_fog_glass,
        Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, material_metal),
        Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, material_marble),
        Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, material_earth),
    ];

    let moving_sphere = {
        let material_moving_sphere = solid_lambert(arena, Color::new_rgb(0.7, 0.3, 0.1));
        let sphere = arena.alloc(Sphere::new(
            Point3::new(400.0, 400.0, 200.0),
            50.0,
            material_moving_sphere,
        ));
        MovingHittable::new(sphere, Dir3::new(30.0, 0.0, 0.0))
    };

    let obj_sphere_cube = {
        let material = solid_lambert(arena, Color::new_rgb(0.75, 0.75, 0.75));
        let mut spheres = Vec::new();
        let rot = RotationAroundUp::new(15.0);
        let dir_1 = rot.apply_dir(Dir3::RIGHT);
        let dir_2 = rot.apply_dir(Dir3::UP);
        let dir_3 = rot.apply_dir(Dir3::FORWARD);
        let origin = Point3::new(-100.0, 270.0, 395.0);
        for _ in 0..1000 {
            let rnd = rng.gen::<[f32; 3]>();
            let p = origin + 165.0 * (dir_1 * rnd[0] + dir_2 * rnd[1] + dir_3 * rnd[2]);
            spheres.push(Sphere::new(p, 10.0, material));
        }
        BoundingVolumeHierarchy::new(spheres, &camera.time_interval)
    };

    let obj_fog = {
        let boundary = arena.alloc(Sphere::new(Point3::ORIGIN, 5000.0, material_fog));
        let material_fog = smoke(arena, Color::new_rgb(1.0, 1.0, 1.0));
        ConstantMedium::new(boundary, material_fog, 0.0001)
    };

    let hittable: Vec<Box<dyn Hittable>> = vec![
        Box::new(boxes),
        Box::new(light),
        Box::new(spheres),
        Box::new(moving_sphere),
        Box::new(obj_fog_sphere),
        Box::new(obj_sphere_cube),
        Box::new(obj_fog),
    ];
    World {
        background: BackgroundColor::Solid {
            color: Color::BLACK,
        },
        camera,
        hittable,
    }
}

pub fn create_world_perlin_spheres<'a>(
    arena: &'a mut bumpalo::Bump,
    rng: &mut common::TRng,
) -> World<impl Hittable + 'a> {
    let camera = Camera::build()
        .vertical_fov(40.0, 3.0 / 4.0)
        .position(Point3::new(13.0, 2.0, 3.0))
        .look_at(Dir3::UP, Point3::new(0.0, 0.0, 0.0))
        .build();

    let texture_noise = arena.alloc(Texture::Marble {
        noise: Perlin::new(8, rng),
        scale: 4.0,
    });
    let material_noise = arena.alloc(Material::Lambert {
        albedo: texture_noise,
    });
    let hittable = vec![
        Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, material_noise),
        Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, material_noise),
    ];
    World {
        background: BackgroundColor::Sky,
        camera,
        hittable,
    }
}

pub fn create_world_cornell_box_smoke<'a>(
    arena: &'a mut bumpalo::Bump,
    _rng: &'a mut common::TRng,
) -> World<impl Hittable + '_> {
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

    let hittable: Vec<Box<dyn Hittable>> = {
        let walls = vec![
            Rect::new_yz(
                Point3::new(0.0, hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                red,
            ),
            Rect::new_yz(
                Point3::new(2.0 * hsize, hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                green,
            ),
            Rect::new_xz(
                Point3::new(hsize, 0.0, hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ),
            Rect::new_xz(
                Point3::new(hsize, 2.0 * hsize, hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ),
            Rect::new_xy(
                Point3::new(hsize, hsize, 2.0 * hsize),
                2.0 * hsize,
                2.0 * hsize,
                white,
            ),
            Rect::new_xz(
                Point3::new(hsize, 2.0 * hsize - 1.0, hsize),
                300.0,
                300.0,
                light,
            ),
        ];

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
                transformation: RotationAroundUp::new(15.0),
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
                transformation: RotationAroundUp::new(-18.0),
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

        vec![Box::new(walls), Box::new(boxes)]
    };

    World {
        background: BackgroundColor::Solid {
            color: Color::BLACK,
        },
        camera,
        hittable,
    }
}
 */
pub fn create_world_cornell_box<'a>(
    wb: &'a WorldBuilder<'a>,
    _rng: &'a mut common::TRng,
) -> World<'a> {
    let hsize = 278.0;
    let camera = Camera::build()
        .vertical_fov(40.0, 1.0)
        .position(Point3::new(hsize, hsize, -800.0))
        .look_at(Dir3::UP, Point3::new(hsize, hsize, 0.0))
        .build();
    let background = BackgroundColor::Solid { color: Color::BLACK };

    let red = wb.material_lambert_solid(Color::new_rgb(0.65, 0.05, 0.05));
    let white = wb.material_lambert_solid(Color::new_rgb(0.73, 0.73, 0.73));
    let green = wb.material_lambert_solid(Color::new_rgb(0.12, 0.45, 0.15));
    let light = wb.material_diffuse_light_solid(Color::new_rgb(15.0, 15.0, 15.0));

    let scene = wb
        .new_group()
        .add(wb.new_obj_rect_yz(
            Point3::new(0.0, hsize, hsize),
            2.0 * hsize,
            2.0 * hsize,
            red,
        ))
        .add(wb.new_obj_rect_yz(
            Point3::new(2.0 * hsize, hsize, hsize),
            2.0 * hsize,
            2.0 * hsize,
            green,
        ))
        .add(wb.new_obj_rect_xz(
            Point3::new(hsize, 0.0, hsize),
            2.0 * hsize,
            2.0 * hsize,
            white,
        ))
        .add(wb.new_obj_rect_xz(
            Point3::new(hsize, 2.0 * hsize, hsize),
            2.0 * hsize,
            2.0 * hsize,
            white,
        ))
        .add(wb.new_obj_rect_xy(
            Point3::new(hsize, hsize, 2.0 * hsize),
            2.0 * hsize,
            2.0 * hsize,
            white,
        ))
        .add(
            wb.new_obj_rect_xz(
                Point3::new(hsize, 2.0 * hsize - 1.0, hsize),
                130.0,
                130.0,
                light,
            )
            .set_all_geo_as_poi(),
        )
        .add(
            wb.new_obj_box(165.0, 330.0, 165.0, white)
                .rotate_around_up(15.0)
                .translate(Dir3::new(265.0, 0.0, 295.0)),
        )
        .add(
            wb.new_obj_box(165.0, 165.0, 165.0, white)
                .rotate_around_up(-18.0)
                .translate(Dir3::new(130.0, 0.0, 65.0)),
        )
        .build();

    scene.finish(wb, background, camera)
}

pub fn create_world_simple_plane<'a>(
    wb: &'a WorldBuilder<'a>,
    _rng: &'a mut common::TRng,
) -> World<'a> {
    let camera = Camera::build()
        .vertical_fov(60.0, 9.0 / 16.0)
        .position(Point3::new(0.0, 6.0, 10.0))
        .look_at(Dir3::UP, Point3::ORIGIN)
        .build();
    let background = BackgroundColor::Solid {
            color: Color::new_rgb(0.1, 0.1, 0.1),
        };

    let mat_emit = wb.material_diffuse_light_solid(100.0 * Color::new_rgb(1.0, 1.0, 1.0));
    let mat_floor = wb.material_lambert_solid(Color::new_rgb(0.0, 0.0, 0.4));

    let scene = wb
        .new_group()
        .add(
            wb.new_obj_rect_xy(Point3::new(0.0, 2.0, 0.0), 1.0, 1.0, mat_emit)
                .set_all_geo_as_poi(),
        )
        .add(wb.new_obj_rect_xz(Point3::ORIGIN, 10.0, 10.0, mat_floor))
        .build();

    scene.finish(wb, background, camera)
}

pub fn create_world_earth_mapped<'a>(
    wb: &'a WorldBuilder<'a>,
    _rng: &'a mut common::TRng,
) -> World<'a> {
    // A single sphere with a image texture
    let camera = Camera::build()
        .vertical_fov(60.0, 16.0 / 19.0)
        .position(Point3::new(0.0, 2.0, 10.0))
        .look_at(Dir3::UP, Point3::ORIGIN)
        .build();
    let background = BackgroundColor::Sky;

    let tex_earth = wb.texture_image_from_file(
        std::path::Path::new("input/earthmap.jpg"),
        image::ImageFormat::Jpeg,
    );
    let mat_earth = wb.material_lambert(tex_earth);

    let scene = wb
        .new_group()
        .add(wb.new_obj_sphere(2.0, mat_earth))
        .build();
    scene.finish(wb, background, camera)
}

pub fn create_world_moving_spheres<'a>(
    wb: &'a WorldBuilder<'a>,
    _rng: &'a mut common::TRng,
) -> World<'a> {
    // One large sphere as ground,
    // One sphere moving fast from left to right
    // One sphere moving fast fro up to down

    let camera = Camera::build()
        .vertical_fov(60.0, 16.0 / 19.0)
        .position(Point3::new(0.0, 2.0, 10.0))
        .look_at(Dir3::UP, Point3::new(0.0, 2.0, 0.0))
        .motion_blur(0.0, 0.5)
        .build();
    let background = BackgroundColor::Sky;

    let tex_black = wb.texture_solid(Color::new_rgb(0.0, 0.0, 0.0));
    let tex_white = wb.texture_solid(Color::new_rgb(1.0, 1.0, 1.0));
    let tex_ground = wb.alloc(Texture::Checker {
        inv_frequency: 10.0,
        even: tex_black,
        odd: tex_white,
    });
    let mat_ground = wb.material_lambert(tex_ground);
    let mat_red = wb.material_lambert_solid(Color::new_rgb(0.6, 0.2, 0.2));
    let mat_blue = wb.material_lambert_solid(Color::new_rgb(0.2, 0.2, 0.6));

    let scene = wb
        .new_group()
        .add(
            wb.new_obj_sphere(100.0, mat_ground)
                .translate(Dir3::new(0.0, -100.0, 0.0)),
        )
        .add(
            wb.new_obj_sphere(0.5, mat_red)
                .translate(Dir3::new(-2.0, 1.5, 0.0))
                .animate_moving(Dir3::new(2.0, 0.0, 0.0)),
        )
        .add(
            wb.new_obj_sphere(0.5, mat_blue)
                .translate(Dir3::new(0.0, 0.5, 0.0))
                .animate_moving(Dir3::new(0.0, 1.0, 0.0)),
        )
        .build();

    scene.finish(wb, background, camera)
}

pub fn create_world_final_scene1<'a>(
    wb: &'a WorldBuilder<'a>,
    rng: &'a mut common::TRng,
) -> World<'a> {
    let camera = Camera::build()
        .vertical_fov(60.0, 9.0 / 16.0)
        .position(Point3::new(13.0, 2.0, 3.0))
        .look_at(Dir3::UP, Point3::ORIGIN)
        .focus_distance(10.0)
        .aperture(0.1)
        .build();
    let background = BackgroundColor::Sky;

    let mat_ground = wb.material_lambert_solid(Color::new_rgb(0.5, 0.5, 0.5));
    let mut scene = wb.new_group();
    let ground_radius = 1000.0;
    let ground_center = Dir3::new(0.0, -ground_radius, 0.0);
    scene = scene.add(wb.new_obj_sphere_ground(ground_radius, 0.0, mat_ground));
    let gen_color = |rng: &mut common::TRng| -> Color {
        Color::new_rgb_arr(rng.gen::<[f32; 3]>())
    };
    let gen_muted_color = |rng: &mut common::TRng| -> Color {
        Color::convolution(gen_color(rng), gen_color(rng))
    };
    let material_glass = wb.material_dielectric(1.5);
    let gen_material = |rng: &mut common::TRng| -> &Material {
        let material_sample = rng.gen::<f32>();
        if material_sample < 0.8 {
            wb.material_lambert_solid(gen_muted_color(rng))
        } else if material_sample < 0.95 {
            wb.material_metal_solid(gen_color(rng), rng.gen_range(0.0..0.5))
        } else {
            material_glass
        }
    };
    let gen_offset = |rng: &mut common::TRng| -> Dir3 {
        Dir3::new(
        rng.gen::<f32>() * 0.9,
        0.0,
        rng.gen::<f32>() * 0.9)
    };
    
    for a in -11..=11 {
        for b in -11..=11 {
            let center = Dir3::new(
                a as f32,
                0.2,
                b as f32,
            ) + gen_offset(rng);
            if (center - Dir3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = gen_material(rng);
                let small_radius = 0.2;
                let real_center = ground_center
                    + (center - ground_center).with_length(ground_radius + small_radius);
                scene = scene.add(wb.new_obj_sphere(small_radius, material).translate(real_center));
            }
        }
    }

    scene = scene.add(wb.new_obj_sphere(1.0, material_glass).translate(Dir3::new(0.0, 1.0, 0.0)));
    let material_big_solid = wb.material_lambert_solid(Color::new_rgb(0.4, 0.2, 0.1));
    scene = scene.add(wb.new_obj_sphere(1.0, material_big_solid).translate(Dir3::new(-4.0, 1.0, 0.0)));
    let material_big_metal = wb.material_metal_solid(Color::new_rgb(0.7, 0.6, 0.5), 0.0);
    scene = scene.add(wb.new_obj_sphere(1.0, material_big_metal).translate(Dir3::new(4.0, 1.0, 0.0)));

    scene.build().finish(wb, background, camera)
}

pub fn create_world_defocus_blur<'a>(
    wb: &'a WorldBuilder<'a>,
    _rng: &'a mut common::TRng,
) -> World<'a> {
    let camera = Camera::build()
        .vertical_fov(60.0, 9.0 / 16.0)
        .position(Point3::ORIGIN + Dir3::BACKWARD * 3.0 + Dir3::UP * 3.0 + 3.0 * Dir3::RIGHT)
        .look_at_focus(Dir3::UP, Point3::ORIGIN + Dir3::FORWARD)
        .aperture(0.1)
        .build();
    let background = BackgroundColor::Sky;

    let material_ground = wb.material_lambert_solid(Color::new_rgb(0.8, 0.8, 0.0));
    let material_center = wb.material_lambert_solid(Color::new_rgb(0.7, 0.3, 0.3));
    let material_left = wb.material_metal_solid(Color::new_rgb(0.6, 0.6, 0.8), 0.05);
    let material_right = wb.material_metal_solid(Color::new_rgb(0.8, 0.6, 0.2), 0.5);
    let material_front = wb.material_dielectric(1.5);

    let scene = wb
        .new_group()
        .add(
            wb.new_obj_sphere(100.0, material_ground)
                .translate(Dir3::DOWN * 100.5),
        )
        .add(
            wb.new_obj_sphere(0.5, material_center)
                .translate(Dir3::FORWARD),
        )
        .add(
            wb.new_obj_sphere(0.5, material_left)
                .translate(Dir3::LEFT + Dir3::FORWARD),
        )
        .add(
            wb.new_obj_sphere(0.5, material_right)
                .translate(Dir3::RIGHT + Dir3::FORWARD),
        )
        .add(
            wb.new_obj_sphere(0.3, material_front)
                .translate(0.5 * Dir3::LEFT + 0.3 * Dir3::UP),
        )
        .build();

    scene.finish(wb, background, camera)
}
