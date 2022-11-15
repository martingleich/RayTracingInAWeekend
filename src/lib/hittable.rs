use std::ops::Range;

use rand::Rng;

use crate::{
    aabb::Aabb,
    common,
    material::Material,
    ray::Ray,
    transformations::Transformation,
    vec2::Vec2f,
    vec3::{Dir3, Point3},
    WorldScatteringDistributionProvider,
};

pub mod rect_geometry;
pub mod sphere_geometry;
pub mod triangle_geometry;
use self::{
    rect_geometry::RectGeometry, sphere_geometry::SphereGeometry,
    triangle_geometry::TriangleGeometry,
};

#[derive(Debug, Clone)]
pub struct GeoHitInteraction {
    pub position: Point3,
    pub normal: Dir3,
    pub uv: Vec2f,
    pub t: f32,
    pub front_face: bool,
}

impl GeoHitInteraction {
    pub fn new_from_ray(
        ray: &Ray,
        position: &Point3,
        surface_normal: &Dir3,
        t: f32,
        uv: Vec2f,
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
            uv,
        }
    }

    pub fn to_hit_interaction<'a>(&self, material: &'a Material<'a>) -> HitInteraction<'a> {
        HitInteraction {
            position: self.position,
            normal: self.normal,
            uv: self.uv,
            t: self.t,
            front_face: self.front_face,
            material,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HitInteraction<'a> {
    pub position: Point3,
    pub normal: Dir3,
    pub uv: Vec2f,
    pub t: f32,
    pub front_face: bool,
    pub material: &'a Material<'a>,
}

impl<'a> HitInteraction<'a> {
    pub fn new_from_ray(
        ray: &Ray,
        position: &Point3,
        surface_normal: &Dir3,
        t: f32,
        material: &'a Material,
        uv: Vec2f,
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
            material,
            uv,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Geometry {
    Sphere(SphereGeometry),
    Rect(RectGeometry),
    AxisAlignedBox(Aabb),
    Triangle(TriangleGeometry),
}

pub enum SceneElement<'a> {
    Group(Vec<&'a SceneElement<'a>>),
    Geometry(Geometry, &'a Material<'a>),
    Animation(&'a SceneElement<'a>, Dir3),
    Transformation(&'a SceneElement<'a>, Transformation),
}

pub struct Scene<'a> {
    root: &'a SceneElement<'a>,
}

impl<'a> Scene<'a> {
    pub fn new(root: &'a SceneElement<'a>) -> Self {
        Self { root }
    }

    pub fn hit(
        &'a self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut rand_xoshiro::Xoroshiro128PlusPlus,
    ) -> Option<HitInteraction> {
        self.root.hit(ray, t_range, rng)
    }
}

impl Geometry {
    pub fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<GeoHitInteraction> {
        match self {
            Geometry::Sphere(geo) => geo.hit(ray, t_range),
            Geometry::Rect(geo) => geo.hit(ray, t_range),
            Geometry::AxisAlignedBox(geo) => geo.hit(ray, t_range),
            Geometry::Triangle(geo) => geo.hit(ray, t_range),
        }
    }

    pub fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        match self {
            Geometry::Sphere(geo) => Some(geo.bounding_box()),
            Geometry::Rect(geo) => Some(geo.bounding_box(0.01)),
            Geometry::AxisAlignedBox(geo) => Some(*geo),
            Geometry::Triangle(geo) => Some(geo.bounding_box()),
        }
    }

    pub fn get_world_scattering_provider(&self) -> Option<WorldScatteringDistributionProvider> {
        match self {
            Geometry::Rect(geo) => Some(WorldScatteringDistributionProvider::Rect(*geo)),
            _ => None,
        }
    }
    pub fn partial_apply_transformation(
        &self,
        transformation: &Transformation,
    ) -> (Geometry, Option<Transformation>) {
        match self {
            Geometry::Sphere(geo) => {
                let center = transformation.apply_point(geo.center);
                let radius = transformation.apply_distance(geo.radius);
                (Geometry::Sphere(SphereGeometry::new(center, radius)), None)
            }
            Geometry::Triangle(geo) => {
                let positions = geo.positions.map(|p| transformation.apply_point(p));
                let normals = geo.normals.map(|n| transformation.apply_normal(n));
                (
                    Geometry::Triangle(TriangleGeometry {
                        positions,
                        normals,
                        texture_coords: geo.texture_coords,
                    }),
                    None,
                )
            }
            geo => (
                *geo,
                if transformation.is_zero() {
                    None
                } else {
                    Some(*transformation)
                },
            ),
        }
    }
}

impl<'a> SceneElement<'a> {
    pub fn hit(
        &'a self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut rand_xoshiro::Xoroshiro128PlusPlus,
    ) -> Option<HitInteraction> {
        match self {
            SceneElement::Group(elements) => {
                let mut t_range_copy = t_range.clone();
                let mut closest: Option<HitInteraction> = None;
                for child in elements {
                    if let Some(hi) = child.hit(ray, &t_range_copy, rng) {
                        t_range_copy.end = hi.t;
                        closest = Some(hi);
                    }
                }
                closest
            }
            SceneElement::Geometry(geo, material) => geo
                .hit(ray, t_range)
                .map(|h| h.to_hit_interaction(material)),
            SceneElement::Transformation(elem, transform) => {
                let ray_transformed = transform.reverse_ray(ray);
                elem.hit(&ray_transformed, t_range, rng)
                    .map(|h| transform.apply_hit_interaction(h))
            }
            SceneElement::Animation(elem, velocity) => {
                let transform = Transformation::ZERO.translate(*velocity * ray.time);
                let ray_transformed = transform.reverse_ray(ray);
                elem.hit(&ray_transformed, t_range, rng)
                    .map(|h| transform.apply_hit_interaction(h))
            }
        }
    }
}

impl Transformation {
    pub fn reverse_ray(&self, ray: &Ray) -> Ray {
        Ray {
            origin: self.reverse_point(ray.origin),
            direction: self.reverse_normal(ray.direction),
            time: ray.time,
        }
    }

    pub fn apply_hit_interaction<'a>(&self, mut hi: HitInteraction<'a>) -> HitInteraction<'a> {
        hi.position = self.apply_point(hi.position);
        hi.normal = self.apply_normal(hi.normal);
        hi
    }

    pub fn apply_aabb(&self, aabb: Aabb) -> Aabb {
        let corners = aabb.corners();
        for mut c in corners {
            self.apply_point_mut(&mut c)
        }
        Aabb::new_surrounding_points(&corners)
    }
}

pub struct ConstantMedium<'a> {
    boundary: &'a Geometry,
    phase_function: &'a Material<'a>,
    neg_inv_density: f32,
}

impl<'a> ConstantMedium<'a> {
    pub fn new(boundary: &'a Geometry, phase_function: &'a Material<'a>, density: f32) -> Self {
        Self {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        }
    }

    pub fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let start_boundary = self
            .boundary
            .hit(ray, &(f32::NEG_INFINITY..f32::INFINITY))?
            .t;
        let next = start_boundary + 0.001;
        let end_boundary = self.boundary.hit(ray, &(next..f32::INFINITY))?.t;

        let mut start_medium = start_boundary.max(t_range.start);
        let end_medium = end_boundary.min(t_range.end);
        if start_medium >= end_medium {
            return None;
        }

        if start_medium < 0.0 {
            start_medium = 0.0;
        }

        let distance_inside_boundary = end_medium - start_medium;
        let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = start_medium + hit_distance;
        Some(HitInteraction {
            position: ray.at(t),
            normal: Dir3::UP, // Arbitrary
            uv: Vec2f::ZERO,  // Undefined
            t,
            front_face: false, // Arbitrary
            material: self.phase_function,
        })
    }

    pub fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.boundary.bounding_box(time_range)
    }
}

/*
#[derive(Default, Debug, Clone, Copy)]
struct BoundingVolumeNode {
    aabb: Aabb,
    axis_id: usize,
    left: usize,
    right: usize,
}
pub struct BoundingVolumeHierarchy<'a> {
    items: Vec<&'a Hittable<'a>>,
    nodes: Vec<BoundingVolumeNode>,
    initial_index: usize,
}

impl<'a> BoundingVolumeHierarchy<'a> {
    pub fn new(items: Vec<&'a Hittable>, time_range: &Range<f32>) -> Self {
        let mut hittables = items
            .iter()
            .map(|h| h.bounding_box(time_range).unwrap())
            .enumerate()
            .map(|mut x| {
                x.0 = usize::MAX - x.0;
                x
            })
            .collect::<Vec<_>>();
        let mut nodes = Vec::<BoundingVolumeNode>::new();
        let (initial_index, _, _) = Self::_new(&mut hittables[..], &mut nodes, 0, 0);
        //eprint!("{depth}");
        Self {
            items,
            nodes,
            initial_index,
        }
    }

    fn _new(
        hittables: &mut [(usize, Aabb)],
        nodes: &mut Vec<BoundingVolumeNode>,
        axis_id: usize,
        depth: usize,
    ) -> (usize, Aabb, usize) {
        if hittables.len() == 1 {
            (hittables[0].0, hittables[0].1, depth)
        } else {
            let comparer =
                |a: &Aabb, b: &Aabb| a.min.0.e[axis_id].partial_cmp(&b.min.0.e[axis_id]).unwrap();
            hittables.sort_by(|a, b| comparer(&a.1, &b.1));
            if hittables.len() == 2 {
                let result_id = nodes.len();
                let aabb = Aabb::new_surrounding_boxes(&[hittables[0].1, hittables[1].1]);
                nodes.push(BoundingVolumeNode {
                    aabb,
                    left: hittables[0].0,
                    right: hittables[1].0,
                    axis_id,
                });
                (result_id, aabb, depth)
            } else {
                let mid = hittables.len() / 2;
                let (left_half, right_half) = hittables.split_at_mut(mid);
                let result_id = nodes.len();
                nodes.push(Default::default());
                let (left_id, left_aabb, left_depth) =
                    Self::_new(left_half, nodes, (axis_id + left_half.len()) % 3, depth + 1);
                let (right_id, right_aabb, right_depth) = Self::_new(
                    right_half,
                    nodes,
                    (axis_id + right_half.len()) % 3,
                    depth + 1,
                );
                let aabb = Aabb::new_surrounding_boxes(&[left_aabb, right_aabb]);
                nodes[result_id] = BoundingVolumeNode {
                    aabb,
                    left: left_id,
                    right: right_id,
                    axis_id,
                };
                (result_id, aabb, left_depth.max(right_depth))
            }
        }
    }

    fn _hit(
        &self,
        node: usize,
        ray: &Ray,
        t_range: &mut Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        if node > self.items.len() {
            self.items[usize::MAX - node].hit(ray, t_range, rng)
        } else if self.nodes[node].aabb.hit(ray, t_range) {
            // Check the sides in the order in which the ray points(i.e. ray points from left to right -> first check left, then right)
            // We still have to check both sides, so we can correctly handle elements on the seperator(TODO: Add these elements to both sides)
            if ray.direction.0.e[self.nodes[node].axis_id] > 0.0 {
                let interaction1 = self._hit(self.nodes[node].left, ray, t_range, rng);
                if let Some(hi) = &interaction1 {
                    t_range.end = hi.t;
                }
                let interaction2 = self._hit(self.nodes[node].right, ray, t_range, rng);
                if let Some(hi) = &interaction2 {
                    t_range.end = hi.t;
                    return interaction2;
                }
                interaction1
            } else {
                let interaction1 = self._hit(self.nodes[node].right, ray, t_range, rng);
                if let Some(hi) = &interaction1 {
                    t_range.end = hi.t;
                }
                let interaction2 = self._hit(self.nodes[node].left, ray, t_range, rng);
                if let Some(hi) = &interaction2 {
                    t_range.end = hi.t;
                    return interaction2;
                }
                interaction1
            }
        } else {
            None
        }
    }

    pub fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let mut t_range = t_range.clone();
        self._hit(self.initial_index, ray, &mut t_range, rng)
    }

    pub fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        if self.initial_index < self.nodes.len() {
            Some(self.nodes[self.initial_index].aabb)
        } else {
            None
        }
    }
}
 */
