use crate::{
    ray::Ray,
    vec3::{Dir3, Point3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitInteraction {
    pub position: Point3,
    pub normal: Dir3,
    pub t: f32,
    pub front_face: bool,
}

impl HitInteraction {
    pub fn new_from_ray(ray: Ray, position: Point3, surface_normal: Dir3, t: f32) -> Self {
        let front_face = Dir3::dot(surface_normal, ray.direction) < 0.0;
        let normal = if front_face {
            surface_normal
        } else {
            -surface_normal
        };
        Self {
            position,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitInteraction>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitInteraction> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Dir3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return None;
        } else {
            let sqrtd = disc.sqrt();

            let root_small = (-half_b - sqrtd) / a;
            let t = if root_small < t_min || root_small > t_max {
                let root_large = (-half_b + sqrtd) / a;
                if root_large < t_min || root_large > t_max {
                    return None;
                } else {
                    root_large
                }
            } else {
                root_small
            };

            let position = ray.at(t);
            let surface_normal = (position - self.center) / self.radius;
            return Some(HitInteraction::new_from_ray(
                ray,
                position,
                surface_normal,
                t,
            ));
        }
    }
}

pub struct HittableList {
    spheres: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
        }
    }
    pub fn push(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitInteraction> {
        self.spheres
            .iter()
            .filter_map(|s| s.hit(ray, t_min, t_max))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
