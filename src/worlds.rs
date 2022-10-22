use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{HittableList, Sphere};

use crate::material::Material;
use crate::size2i::Size2i;
use crate::vec3::{Dir3, Point3};

pub fn create_world_defocus_blur(image_size: Size2i) -> (Camera, HittableList) {
    let viewport_width = 1.2;
    let viewport_height = image_size.aspect_ratio() * viewport_width;
    let camera = Camera::new_look_at(
        viewport_width,
        viewport_height,
        Point3::ORIGIN + Dir3::BACKWARD * 3.0 + Dir3::UP * 3.0 + 3.0*Dir3::RIGHT,
        Dir3::UP,
        Point3::ORIGIN + Dir3::FORWARD,
        0.1
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