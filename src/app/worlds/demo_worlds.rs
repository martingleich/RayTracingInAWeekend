use rand::Rng;
use ray_tracing_in_a_weekend::*;

use super::world_builder::{NodeRef, WorldBuilder};

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

pub fn create_world_final_scene2<'a>(
    wb: &'a WorldBuilder<'a>,
    rng: &'a mut common::TRng,
) -> World<'a> {
    let camera = Camera::build()
        .vertical_fov(40.0, 1.0)
        .position(Point3::new(478.0, 278.0, -600.0))
        .look_at(Dir3::UP, Point3::new(278.0, 278.0, 0.0))
        .motion_blur(0.0, 1.0)
        .build();
    let background = BackgroundColor::Solid {
        color: Color::BLACK,
    };

    let mat_ground = wb.material_lambert_solid(Color::new_rgb(0.48, 0.83, 0.53));
    let ground = {
        let mut group = wb.new_group();
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
                let min =
                    range.min + Dir3::new(i as f32 * width, 0.0, j as f32 * depth) - Point3::ORIGIN;
                group = group.add(
                    wb.new_obj_box(width, height * rng.gen::<f32>(), depth, mat_ground)
                        .translate(min),
                )
            }
        }
        group.build()
    };

    let mat_light = wb.material_diffuse_light_solid(Color::new_rgb(7.0, 7.0, 7.0));
    let light = wb
        .new_obj_rect_xz(Point3::new(273.0, 554.0, 279.0), 300.0, 300.0, mat_light)
        .build();

    let mat_glass = wb.material_dielectric(1.5);
    let mat_metal = wb.material_metal_solid(Color::new_rgb(0.8, 0.8, 0.9), 1.0);
    let tex_marble = wb.texture_marble(0.1, rng);
    let mat_marble = wb.material_lambert(tex_marble);
    let tex_earth = wb.texture_image_from_file(
        std::path::Path::new("input/earthmap.jpg"),
        image::ImageFormat::Jpeg,
    );
    let mat_earth = wb.material_lambert(tex_earth);
    let mat_fog = wb.material_isotropic_solid(Color::new_rgb(0.2, 0.4, 0.9));
    let mat_moving_sphere = wb.material_lambert_solid(Color::new_rgb(0.7, 0.3, 0.1));

    let floating_spheres = wb
        .new_group()
        .add(
            wb.new_obj_sphere(70.0, mat_glass)
                .translate(Dir3::new(360.0, 150.0, 145.0)),
        )
        .add(
            wb.new_obj_sphere(70.0, mat_fog)
                .translate(Dir3::new(360.0, 150.0, 145.0))
                .set_all_geo_densitity(0.2),
        )
        .add(
            wb.new_obj_sphere(50.0, mat_glass)
                .translate(Dir3::new(260.0, 150.0, 45.0)),
        )
        .add(
            wb.new_obj_sphere(50.0, mat_metal)
                .translate(Dir3::new(0.0, 150.0, 145.0)),
        )
        .add(
            wb.new_obj_sphere(80.0, mat_marble)
                .translate(Dir3::new(220.0, 280.0, 300.0)),
        )
        .add(
            wb.new_obj_sphere(100.0, mat_earth)
                .translate(Dir3::new(400.0, 200.0, 400.0)),
        )
        .add(
            wb.new_obj_sphere(50.0, mat_moving_sphere)
                .translate(Dir3::new(400.0, 400.0, 200.0))
                .animate_moving(Dir3::new(30.0, 0.0, 0.0)),
        )
        .build();

    let obj_sphere_cube = {
        let material = wb.material_lambert_solid(Color::new_rgb(0.75, 0.75, 0.75));
        let mut spheres = wb.new_group();
        let rot = Transformation::ZERO.rotate_around_up(15.0);
        let dir_1 = rot.apply_direction(Dir3::RIGHT);
        let dir_2 = rot.apply_direction(Dir3::UP);
        let dir_3 = rot.apply_direction(Dir3::FORWARD);
        let origin = Dir3::new(-100.0, 270.0, 395.0);
        for _ in 0..1000 {
            let rnd = rng.gen::<[f32; 3]>();
            let p = origin + 165.0 * (dir_1 * rnd[0] + dir_2 * rnd[1] + dir_3 * rnd[2]);
            spheres = spheres.add(wb.new_obj_sphere(10.0, material).translate(p));
        }
        spheres.build()
    };

    let obj_fog = {
        let mat_fog = wb.material_isotropic_solid(Color::new_rgb(1.0, 1.0, 1.0));
        wb.new_obj_sphere(5000.0, mat_fog)
            .set_all_geo_densitity(0.0001)
            .build()
    };

    let scene = wb
        .new_group()
        .add(obj_fog)
        .add(ground)
        .add(obj_sphere_cube)
        .add(floating_spheres)
        .add(light)
        .build();

    scene.finish(wb, background, camera)
}

pub fn create_world_perlin_spheres<'a>(
    wb: &'a WorldBuilder<'a>,
    rng: &'a mut common::TRng,
) -> World<'a> {
    let camera = Camera::build()
        .vertical_fov(40.0, 3.0 / 4.0)
        .position(Point3::new(13.0, 2.0, 3.0))
        .look_at(Dir3::UP, Point3::new(0.0, 0.0, 0.0))
        .build();
    let background = BackgroundColor::Sky;

    let tex_noise = wb.texture_marble(4.0, rng);
    let mat_noise = wb.material_lambert(tex_noise);
    let scene = wb
        .new_group()
        .add(wb.new_obj_sphere_ground(1000.0, 0.0, mat_noise))
        .add(
            wb.new_obj_sphere(2.0, mat_noise)
                .translate(Dir3::new(0.0, 2.0, 0.0)),
        )
        .build();

    scene.finish(wb, background, camera)
}

pub fn create_world_cornell_box_smoke<'a>(
    wb: &'a WorldBuilder<'a>,
    _rng: &'a mut common::TRng,
) -> World<'a> {
    let hsize = 278.0;
    let camera = Camera::build()
        .vertical_fov(40.0, 1.0)
        .position(Point3::new(hsize, hsize, -800.0))
        .look_at(Dir3::UP, Point3::new(hsize, hsize, 0.0))
        .build();
    let background = BackgroundColor::Solid {
        color: Color::BLACK,
    };

    let smoke_black = wb.material_isotropic_solid(Color::new_rgb(0.0, 0.0, 0.0));
    let smoke_white = wb.material_isotropic_solid(Color::new_rgb(1.0, 1.0, 1.0));
    let cornell_box = create_cornell_box_node(wb, 130.0);

    let scene = wb
        .new_group()
        .add(&cornell_box)
        .add(
            wb.new_obj_box(165.0, 330.0, 165.0, smoke_white)
                .rotate_around_up(15.0)
                .translate(Dir3::new(265.0, 0.0, 295.0))
                .set_all_geo_densitity(0.01),
        )
        .add(
            wb.new_obj_box(165.0, 165.0, 165.0, smoke_black)
                .rotate_around_up(-18.0)
                .translate(Dir3::new(130.0, 0.0, 65.0))
                .set_all_geo_densitity(0.01),
        )
        .build();
    scene.finish(wb, background, camera)
}

pub fn create_cornell_box_node<'a>(wb: &'a WorldBuilder<'a>, light_size: f32) -> NodeRef<'a> {
    let red = wb.material_lambert_solid(Color::new_rgb(0.65, 0.05, 0.05));
    let white = wb.material_lambert_solid(Color::new_rgb(0.73, 0.73, 0.73));
    let green = wb.material_lambert_solid(Color::new_rgb(0.12, 0.45, 0.15));
    let light = wb.material_diffuse_light_solid(Color::new_rgb(15.0, 15.0, 15.0));
    let hsize = 278.0;

    wb.new_group()
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
                light_size,
                light_size,
                light,
            )
            .set_all_geo_as_poi(),
        )
        .build()
}

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
    let white = wb.material_lambert_solid(Color::new_rgb(0.73, 0.73, 0.73));
    let background = BackgroundColor::Solid {
        color: Color::BLACK,
    };
    let cornell_box = create_cornell_box_node(wb, 130.0);
    let scene = wb
        .new_group()
        .add(&cornell_box)
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
    let tex_ground = wb.texture_checker(10.0, tex_black, tex_white);
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
    let gen_color = |rng: &mut common::TRng| -> Color { Color::new_rgb_arr(rng.gen::<[f32; 3]>()) };
    let gen_muted_color =
        |rng: &mut common::TRng| -> Color { Color::convolution(gen_color(rng), gen_color(rng)) };
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
        Dir3::new(rng.gen::<f32>() * 0.9, 0.0, rng.gen::<f32>() * 0.9)
    };

    for a in -11..=11 {
        for b in -11..=11 {
            let center = Dir3::new(a as f32, 0.2, b as f32) + gen_offset(rng);
            if (center - Dir3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = gen_material(rng);
                let small_radius = 0.2;
                let real_center = ground_center
                    + (center - ground_center).with_length(ground_radius + small_radius);
                scene = scene.add(
                    wb.new_obj_sphere(small_radius, material)
                        .translate(real_center),
                );
            }
        }
    }

    scene = scene.add(
        wb.new_obj_sphere(1.0, material_glass)
            .translate(Dir3::new(0.0, 1.0, 0.0)),
    );
    let material_big_solid = wb.material_lambert_solid(Color::new_rgb(0.4, 0.2, 0.1));
    scene = scene.add(
        wb.new_obj_sphere(1.0, material_big_solid)
            .translate(Dir3::new(-4.0, 1.0, 0.0)),
    );
    let material_big_metal = wb.material_metal_solid(Color::new_rgb(0.7, 0.6, 0.5), 0.0);
    scene = scene.add(
        wb.new_obj_sphere(1.0, material_big_metal)
            .translate(Dir3::new(4.0, 1.0, 0.0)),
    );

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
