use std::ops::Range;

use crate::{
    math,
    ray::Ray,
    vec3::{Dir3, Point3},
};
#[derive(Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

impl Aabb {
    pub fn new_corners(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }
    pub fn new_radius(center: Point3, radius: f32) -> Self {
        Self::new_corners(
            center - Dir3::new(radius, radius, radius),
            center + Dir3::new(radius, radius, radius),
        )
    }
    pub fn new_surrounding_boxes(boxes: &[Aabb]) -> Self {
        let mut min = boxes[0].min.0.e;
        let mut max = boxes[0].max.0.e;
        for b in &boxes[1..] {
            min = math::min_array(min, b.min.0.e);
            max = math::max_array(max, b.max.0.e);
        }
        Self
        {
            min: Point3::new_from_arr(min),
            max: Point3::new_from_arr(max),
        }
    }
    pub fn new_surrounding_points(points: &[Point3]) -> Self {
        let mut min = points[0].0.e;
        let mut max = points[0].0.e;
        for p in &points[1..] {
            min = math::min_array(min, p.0.e);
            max = math::max_array(max, p.0.e);
        }
        Self
        {
            min: Point3::new_from_arr(min),
            max: Point3::new_from_arr(max),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> bool {
        for i in 0..3 {
            let (t0, t1) = math::minmax(
                (self.min.0.e[i] - ray.origin.0.e[i]) / ray.direction.0.e[i],
                (self.max.0.e[i] - ray.origin.0.e[i]) / ray.direction.0.e[i],
            );
            let t_min = t0.max(t_range.start);
            let t_max = t1.min(t_range.end);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }

    pub fn translate(&self, offset: Dir3) -> Aabb {
        Self::new_corners(self.min + offset, self.max + offset)
    }
    pub fn right(&self) -> Dir3 {
        Dir3::new(self.max.0.e[0] - self.min.0.e[0], 0.0, 0.0)
    }
    pub fn up(&self) -> Dir3 {
        Dir3::new(0.0, self.max.0.e[1] - self.min.0.e[1], 0.0)
    }
    pub fn forward(&self) -> Dir3 {
        Dir3::new(0.0, 0.0, self.max.0.e[2] - self.min.0.e[2])
    }
    pub fn corners(&self) -> [Point3; 8] {
        [
            self.min,
            self.min + self.right(),
            self.min + self.up(),
            self.min + self.forward(),
            self.max,
            self.max - self.right(),
            self.max - self.up(),
            self.max - self.forward(),
        ]
    }
}
