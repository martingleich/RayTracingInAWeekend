use std::ops::Range;

use crate::{Aabb, Dir3, Point3, Ray, Vec2f};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SphereGeometry {
    pub center: Point3,
    pub radius: f32,
}

impl SphereGeometry {
    pub fn new(center: Point3, radius: f32) -> Self {
        assert!(radius >= 0.0);
        Self { center, radius }
    }

    pub fn bounding_box(&self) -> Aabb {
        Aabb::new_radius(self.center, self.radius)
    }

    pub fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<(Point3, Dir3, f32, Vec2f)> {
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
            let uv = Self::get_sphere_uv(surface_normal);

            Some((position, surface_normal, root, uv))
        }
    }

    fn get_sphere_uv(pos: Dir3) -> Vec2f {
        let (theta, phi, _) = pos.to_radian();
        Vec2f::new(theta, phi)
    }
}
