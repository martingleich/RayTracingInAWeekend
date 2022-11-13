use std::ops::Range;

use crate::{Aabb, Dir3, Point3, Ray, Vec2f, GeoHitInteraction};

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

    pub fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<GeoHitInteraction> {
        let oc = ray.origin - self.center;
        let half_b = Dir3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - c;
        if disc < 0.0 {
            None
        } else {
            let sqrtd = disc.sqrt();
            let root_small = -half_b - sqrtd;
            let t: f32;
            if t_range.contains(&root_small) {
                t = root_small;
            } else {
                let root_large = -half_b + sqrtd;
                if t_range.contains(&root_large) {
                    t = root_large;
                } else {
                    return None;
                }
            }
            let position = ray.at(t);
            let surface_normal = (position - self.center) / self.radius;
            let uv = Self::get_sphere_uv(surface_normal);

            Some(GeoHitInteraction::new_from_ray(ray, &position, &surface_normal, t, uv))
        }
    }

    fn get_sphere_uv(pos: Dir3) -> Vec2f {
        let (theta, phi, _) = pos.to_radian();
        Vec2f::new(theta, phi)
    }
}
