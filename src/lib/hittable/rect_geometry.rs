use std::ops::Range;

use rand::Rng;

use crate::{common, Dir3, Point3, Ray, Vec2f};

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

#[derive(Debug, Clone, PartialEq)]
pub struct RectGeometry {
    pub rect_plane: RectPlane,
    pub dist: f32,
    pub r1: Range<f32>,
    pub r2: Range<f32>,
}

impl RectGeometry {
    pub fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<(Point3, Dir3, f32, Vec2f)> {
        let (p0, p1, n) = self.rect_plane.get_axis();
        let t = (self.dist - ray.origin.0.e[n]) / ray.direction.0.e[n];
        if t_range.contains(&t) {
            let position = ray.origin + t * ray.direction;
            if self.r1.contains(&position.0.e[p0]) && self.r2.contains(&position.0.e[p1]) {
                let uv = Vec2f::new(
                    (position.0.e[p0] - self.r1.start) / (self.r1.end - self.r1.start),
                    (position.0.e[p1] - self.r2.start) / (self.r1.end - self.r2.start),
                );
                let mut surface_normal = Dir3::ZERO;
                surface_normal.0.e[n] = -1.0;
                return Some((position, surface_normal, t, uv));
            }
        }
        None
    }
    pub fn generate(&self, origin: Point3, rng: &mut common::TRng) -> Dir3 {
        let (p0, p1, n) = self.rect_plane.get_axis();
        let mut e = [0.0; 3];
        e[p0] = rng.gen_range(self.r1.clone());
        e[p1] = rng.gen_range(self.r2.clone());
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
            let area = (self.r1.end - self.r1.start) * (self.r2.end - self.r2.start);
            let distance_squared = hi.2 * hi.2;
            let cosine = Dir3::dot(hi.1, direction).abs();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
}
