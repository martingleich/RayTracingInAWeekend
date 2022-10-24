use std::ops::Range;

use crate::{
    ray::Ray,
    vec3::{Dir3, Point3},
};
#[derive(Clone, Copy, PartialEq)]
pub struct Aabb {
    pub min: Point3,
    pub max: Point3,
}

fn min_array(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0].min(b[0]), a[1].min(b[1]), a[1].min(b[1])]
}
fn max_array(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [a[0].max(b[0]), a[1].max(b[1]), a[1].max(b[1])]
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
    pub fn new_surrounding_boxes(a: &Aabb, b: &Aabb) -> Self {
        Self::new_corners(
            Point3::new_from_arr(min_array(a.min.0.e, b.min.0.e)),
            Point3::new_from_arr(max_array(a.max.0.e, b.max.0.e)),
        )
    }
    pub fn new_surrounding_points(points: &[Point3]) -> Self {
        let mut min = points[0].0.e;
        let mut max = points[0].0.e;
        for p in &points[1..] {
            min = min_array(min, p.0.e);
            max = max_array(max, p.0.e);
        }
        Self::new_corners(Point3::new_from_arr(min), Point3::new_from_arr(max))
    }

    pub fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> bool {
        for i in 0..3 {
            let (t0, t1) = Self::minmax(
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
    fn minmax(a: f32, b: f32) -> (f32, f32) {
        if a < b {
            (a, b)
        } else {
            (b, a)
        }
    }

    pub fn translate(&self, offset: Dir3) -> Aabb {
        Self::new_corners(self.min + offset, self.max + offset)
    }
}
