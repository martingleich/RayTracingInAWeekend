use std::ops::Range;

use crate::{common, Aabb, Dir3, HitInteraction, Hittable, Material, Point3, Ray, Vec2f};

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Point3,
    pub normal: Dir3,
    pub uv: Vec2f,
}
enum BspNodeRef {
    Index(usize),
    Triangles(Vec<[usize; 3]>),
}
struct BspNode {
    aabb: Aabb,
    axis: Dir3,
    left: BspNodeRef,
    right: BspNodeRef,
}
pub struct Mesh<'a> {
    vertices: Vec<Vertex>,

    nodes: Vec<BspNode>,
    root_node: BspNodeRef,

    aabb: Aabb,

    material: &'a Material<'a>,
}

impl<'a> Mesh<'a> {
    pub fn new(
        vertices: Vec<Vertex>,
        triangles: Vec<[usize; 3]>,
        material: &'a Material<'a>,
    ) -> Mesh<'a> {
        let aabb = {
            let points = vertices.iter().map(|v| v.position).collect::<Vec<_>>();
            Aabb::new_surrounding_points(points.as_slice())
        };
        Mesh {
            vertices,
            nodes: Vec::new(),
            root_node: BspNodeRef::Triangles(triangles),
            material,
            aabb,
        }
    }
    fn hit_ref(
        &self,
        node_ref: &BspNodeRef,
        ray: &Ray,
        t_range: &mut Range<f32>,
    ) -> Option<HitInteraction> {
        match node_ref {
            BspNodeRef::Index(idx) => self.hit_node(&self.nodes[*idx], ray, t_range),
            BspNodeRef::Triangles(tris) => {
                let mut result = None;
                for tri in tris {
                    if let Some(hi) = self.hit_triangle(tri, ray, t_range) {
                        t_range.end = hi.t;
                        result = Some(hi);
                    }
                }
                result
            }
        }
    }

    fn hit_node(
        &self,
        node: &BspNode,
        ray: &Ray,
        t_range: &mut Range<f32>,
    ) -> Option<HitInteraction> {
        if !node.aabb.hit(ray, t_range) {
            return None;
        }

        // Check the sides in the order in which the ray points(i.e. ray points from left to right -> first check left, then right)
        let nodes = if Dir3::dot(ray.direction, node.axis) > 0.0 {
            [&node.left, &node.right]
        } else {
            [&node.right, &node.left]
        };
        for node_ref in nodes {
            if let Some(hi) = self.hit_ref(node_ref, ray, t_range) {
                t_range.end = hi.t;
                return Some(hi); // We can stop immediatly since all nodes on the other side are farther away.
            }
        }
        None
    }

    fn hit_triangle(
        &self,
        tri: &[usize; 3],
        ray: &Ray,
        t_range: &mut Range<f32>,
    ) -> Option<HitInteraction> {
        let v0 = &self.vertices[tri[0]];
        let v1 = &self.vertices[tri[1]];
        let v2 = &self.vertices[tri[2]];
        let p0 = v0.position;
        let p1 = v1.position;
        let p2 = v2.position;
        let dir1 = p1 - p0;
        let dir2 = p2 - p0;
        let normal = Dir3::cross(dir1, dir2).unit();
        let d = Dir3::dot(p0 - Point3::ORIGIN, -normal);
        let denom = Dir3::dot(ray.direction, normal);
        if denom.abs() > 0.0001 {
            let t = -(Dir3::dot(normal, ray.origin - Point3::ORIGIN) + d) / denom;
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
                        let uv = v0.uv * w0 + v1.uv * w1 + v2.uv * w2;
                        let surface_normal = v0.normal * w0 + v1.normal * w1 + v2.normal * w2;
                        return Some(HitInteraction::new_from_ray(
                            ray,
                            &position,
                            &surface_normal,
                            t,
                            self.material,
                            uv,
                        ));
                    }
                }
            };
        };
        None
    }
}

impl<'a> Hittable for Mesh<'a> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        _rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let mut t_range = t_range.clone();
        if self.aabb.hit(ray, &t_range) {
            self.hit_ref(&self.root_node, ray, &mut t_range)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        Some(self.aabb)
    }
}
