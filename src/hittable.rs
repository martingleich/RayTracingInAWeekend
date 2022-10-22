use std::ops::Range;

use crate::{
    ray::Ray,
    vec3::{Dir3, Point3},
    Material,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitInteraction {
    pub position: Point3,
    pub normal: Dir3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

impl HitInteraction {
    pub fn new_from_ray(
        ray: &Ray,
        position: &Point3,
        surface_normal: &Dir3,
        t: f32,
        material: &Material,
    ) -> Self {
        let front_face = Dir3::dot(*surface_normal, ray.direction) < 0.0;
        let normal = if front_face {
            *surface_normal
        } else {
            -*surface_normal
        };
        Self {
            position: *position,
            normal,
            t,
            front_face,
            material: *material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        let oc = ray.origin - self.center;
        let half_b = Dir3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - c;
        if disc < 0.0 {
            None
        } else {
            let sqrtd = disc.sqrt();
            let root_small = -half_b - sqrtd;
            let root: f32;
            if t_range.contains(&root_small) {
                root = root_small;
            } else {
                let root_large = -half_b + sqrtd;
                if t_range.contains(&root_large) {
                    root = root_large;
                } else {
                    return None;
                }
            }
            let position = ray.at(root);
            let surface_normal = (position - self.center) / self.radius;
            Some(HitInteraction::new_from_ray(
                ray,
                &position,
                &surface_normal,
                root,
                &self.material,
            ))
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
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        let mut range = t_range.clone();
        let mut min_interaction : Option<HitInteraction> = None;
        for sph in &self.spheres {
            if let Some(hi) = sph.hit(ray, &range) {
                range.end = hi.t;
                min_interaction = Some(hi);
            }
        }
        min_interaction
    }
}
