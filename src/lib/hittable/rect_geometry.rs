use std::ops::Range;

use rand::Rng;

use crate::{common, Dir3, Point3, Ray, Vec2f, GeoHitInteraction, Aabb};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RectPlane {
    Xy,
    Xz,
    Yz,
}

impl RectPlane {
    pub fn get_axis(&self) -> (usize, usize, usize) {
        match self {
            RectPlane::Xy => (0, 1, 2),
            RectPlane::Xz => (0, 2, 1),
            RectPlane::Yz => (1, 2, 0),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectGeometry {
    pub rect_plane: RectPlane,
    pub dist: f32,
    pub r0: (f32, f32),
    pub r1: (f32, f32),
}

impl RectGeometry {
    pub fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<GeoHitInteraction> {
        let (p0, p1, n) = self.rect_plane.get_axis();
        let t = (self.dist - ray.origin.0.e[n]) / ray.direction.0.e[n];
        if t_range.contains(&t) {
            let position = ray.origin + t * ray.direction;
            if position.0.e[p0] >= self.r0.0 && self.r0.0 <= position.0.e[p0] && position.0.e[p1] >= self.r1.0 && self.r1.0 <= position.0.e[p1] {
                let uv = Vec2f::new(
                    (position.0.e[p0] - self.r0.0) / (self.r0.1 - self.r0.0),
                    (position.0.e[p1] - self.r1.0) / (self.r0.1 - self.r1.0),
                );
                let mut surface_normal = Dir3::ZERO;
                surface_normal.0.e[n] = -1.0;
                return Some(GeoHitInteraction::new_from_ray(ray, &position, &surface_normal, t, uv));
            }
        }
        None
    }
    pub fn generate(&self, origin: Point3, rng: &mut common::TRng) -> Dir3 {
        let (p0, p1, n) = self.rect_plane.get_axis();
        let mut e = [0.0; 3];
        e[p0] = rng.gen_range(self.r0.0..self.r0.1);
        e[p1] = rng.gen_range(self.r1.0..self.r1.1);
        e[n] = self.dist;
        (Point3(crate::Vec3 { e }) - origin).unit()
    }
    pub fn value(&self, origin: Point3, direction: Dir3) -> f32 {
        if let Some(hi) = self.hit(
            &Ray {
                origin,
                direction,
                time: 0.0,
            },
            &(0.001..f32::INFINITY),
        ) {
            let area = (self.r0.1 - self.r0.0) * (self.r1.1 - self.r1.0);
            let distance_squared = hi.t * hi.t;
            let cosine = Dir3::dot(hi.normal, direction).abs();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }

    pub fn bounding_box(&self, thickness : f32) -> Aabb {
        let (p0, p1, n) = self.rect_plane.get_axis();
        let mut min = Point3::ORIGIN;
        let mut max = Point3::ORIGIN;
        min.0.e[p0] = self.r0.0;
        min.0.e[p1] = self.r1.0;
        min.0.e[n] = self.dist - thickness;
        max.0.e[p0] = self.r0.1;
        max.0.e[p1] = self.r1.1;
        max.0.e[n] = self.dist + thickness;
        Aabb::new_corners(min, max)
    }
}