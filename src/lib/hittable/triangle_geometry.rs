use std::ops::Range;

use crate::{Point3, Dir3, GeoHitInteraction, Ray, Vec2f, math, Aabb};

#[derive(Debug, Clone, Copy)]
pub struct TriangleGeometry {
    pub positions : [Point3; 3],
    pub normals : [Dir3; 3],
    pub texture_coords : [Vec2f; 3],
}

impl TriangleGeometry {
    pub fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
    ) -> Option<GeoHitInteraction> {
        let [p0, p1, p2] = self.positions;
        let dir1 = p1 - p0;
        let dir2 = p2 - p0;
        let normal = Dir3::cross(dir1, dir2).unit();
        let denom = Dir3::dot(ray.direction, normal);
        if denom.abs() > 0.0001 {
            let t = Dir3::dot(p0 - ray.origin, normal) / denom;
            if t_range.contains(&t) {
                let position = ray.at(t);
                let q = position - p0;
                let v_temp = Dir3::cross(normal, dir2);
                let w1 = Dir3::dot(q, v_temp) / Dir3::dot(dir1, v_temp);
                if w1 > 0.0 && w1 < 1.0 {
                    let v_temp = Dir3::cross(normal, dir1);
                    let w2 = Dir3::dot(q, v_temp) / Dir3::dot(dir2, v_temp);
                    let w0 = 1.0 - w1 - w2;
                    if w2 > 0.0 && w0 > 0.0 {
                        let uv = math::interpolate(w0, w1, w2, &self.texture_coords);
                        let surface_normal = math::interpolate(w0, w1, w2, &self.normals);
                        return Some(GeoHitInteraction::new_from_ray(
                            ray,
                            &position,
                            &surface_normal,
                            t,
                            uv,
                        ));
                    }
                }
            };
        };
        None
    }

    pub fn bounding_box(&self) -> Aabb {
        Aabb::new_surrounding_points(&self.positions)
    }
}