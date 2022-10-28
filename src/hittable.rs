use std::ops::Range;

use rand::Rng;

use crate::{
    aabb::Aabb,
    common,
    ray::Ray,
    transformations::Transformation,
    vec2::Vec2f,
    vec3::{Dir3, Point3},
    Material,
};

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

pub trait Hittable: Send + Sync {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction>;
    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb>;
}

#[derive(Debug, Clone)]
pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f32,
    pub material: &'a Material<'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Point3, radius: f32, material: &'a Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

fn get_sphere_uv(pos: Dir3) -> Vec2f {
    let (theta, phi, _) = pos.to_radian();
    Vec2f::new(theta, phi)
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        _rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
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
            let uv = get_sphere_uv(surface_normal);

            Some(HitInteraction::new_from_ray(
                ray,
                &position,
                &surface_normal,
                root,
                self.material,
                uv,
            ))
        }
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        Some(Aabb::new_radius(self.center, self.radius))
    }
}

pub enum RectPlane {
    Xy,
    Xz,
    Yz,
}
pub struct Rect<'a> {
    rect_plane: RectPlane,
    dist: f32,
    r1: Range<f32>,
    r2: Range<f32>,
    material: &'a Material<'a>,
}

impl<'a> Rect<'a> {
    pub fn new_xy(center: Point3, width: f32, height: f32, material: &'a Material<'a>) -> Self {
        Self {
            rect_plane: RectPlane::Xy,
            dist: center.0.e[2],
            r1: (center.0.e[0] - width * 0.5)..(center.0.e[0] + width * 0.5),
            r2: (center.0.e[1] - height * 0.5)..(center.0.e[1] + height * 0.5),
            material,
        }
    }
    pub fn new_xz(center: Point3, width: f32, depth: f32, material: &'a Material<'a>) -> Self {
        Self {
            rect_plane: RectPlane::Xz,
            dist: center.0.e[1],
            r1: (center.0.e[0] - width * 0.5)..(center.0.e[0] + width * 0.5),
            r2: (center.0.e[2] - depth * 0.5)..(center.0.e[2] + depth * 0.5),
            material,
        }
    }
    pub fn new_yz(center: Point3, height: f32, depth: f32, material: &'a Material<'a>) -> Self {
        Self {
            rect_plane: RectPlane::Yz,
            dist: center.0.e[0],
            r1: (center.0.e[1] - height * 0.5)..(center.0.e[1] + height * 0.5),
            r2: (center.0.e[2] - depth * 0.5)..(center.0.e[2] + depth * 0.5),
            material,
        }
    }
}

impl<'a> Hittable for Rect<'a> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        _rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let (p0, p1, n) = match self.rect_plane {
            RectPlane::Xy => (0, 1, 2),
            RectPlane::Xz => (0, 2, 1),
            RectPlane::Yz => (1, 2, 0),
        };
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
        None
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        None
    }
}

pub struct MovingHittable<'a, T: Hittable> {
    velocity: Dir3,
    hittable: &'a T,
}

impl<'a, T: Hittable> MovingHittable<'a, T> {
    pub fn new(hittable: &'a T, velocity: Dir3) -> Self {
        Self { hittable, velocity }
    }
}

impl<'a, T: Hittable> Hittable for MovingHittable<'a, T> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        // Instead of transforming the object the just move the ray backward
        let mut moved_ray = *ray;
        moved_ray.origin -= self.velocity * ray.time;
        self.hittable.hit(&moved_ray, t_range, rng).map(|mut f| {
            f.position += self.velocity * ray.time;
            f
        })
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        let start_box = self
            .hittable
            .bounding_box(&(time_range.start..time_range.start))?
            .translate(self.velocity * time_range.start);
        let end_box = self
            .hittable
            .bounding_box(&(time_range.end..time_range.end))?
            .translate(self.velocity * time_range.end);
        Some(Aabb::new_surrounding_boxes(&[start_box, end_box]))
    }
}

pub struct AxisAlignedBox<'a> {
    aabb: Aabb,
    material: &'a Material<'a>,
}

impl<'a> AxisAlignedBox<'a> {
    pub fn new(aabb: &Aabb, material: &'a Material<'a>) -> Self {
        Self {
            aabb: *aabb,
            material,
        }
    }
}

impl<'a> Hittable for AxisAlignedBox<'a> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        _rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let ((near_t, near_plane), (far_t, far_plane)) =
            self.aabb.intersections_line(ray.origin, ray.direction)?;
        let (t, plane) = if t_range.contains(&near_t) {
            (near_t, near_plane)
        } else if t_range.contains(&far_t) {
            (far_t, far_plane)
        } else {
            return None;
        };
        let position = ray.origin + t * ray.direction;
        let dir = (position.0.e[plane]
            - 0.5 * (self.aabb.max.0.e[plane] - self.aabb.min.0.e[plane]))
            .signum();

        let mut surface_normal = Dir3::ZERO;
        surface_normal.0.e[plane] = dir;
        return Some(HitInteraction::new_from_ray(
            ray,
            &position,
            &surface_normal,
            t,
            self.material,
            Vec2f::ZERO,
        ));
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        Some(self.aabb)
    }
}

pub struct TransformedHittable<'a, THit: Hittable, TTrans: Transformation> {
    pub hittable: &'a THit,
    pub transformation: TTrans,
}

impl<'a, THit: Hittable, TTrans: Transformation> Hittable
    for TransformedHittable<'a, THit, TTrans>
{
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        // Instead of transforming the object the just move the ray backward
        let mut moved_ray = *ray;
        self.transformation.reverse_point(&mut moved_ray.origin);
        self.transformation.reverse_dir(&mut moved_ray.direction);
        self.hittable.hit(&moved_ray, t_range, rng).map(|mut f| {
            self.transformation.apply_point(&mut f.position);
            self.transformation.apply_dir(&mut f.normal);
            f
        })
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.hittable.bounding_box(time_range).map(|b| {
            let corners = b.corners();
            for mut c in corners {
                self.transformation.apply_point(&mut c)
            }
            Aabb::new_surrounding_points(&corners)
        })
    }
}

pub struct ConstantMedium<'a, T: Hittable> {
    boundary: &'a T,
    phase_function: &'a Material<'a>,
    neg_inv_density: f32,
}

impl<'a, T: Hittable> ConstantMedium<'a, T> {
    pub fn new(boundary: &'a T, phase_function: &'a Material<'a>, density: f32) -> Self {
        Self {
            boundary,
            phase_function,
            neg_inv_density: -1.0 / density,
        }
    }
}

impl<'a, T: Hittable> Hittable for ConstantMedium<'a, T> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let start_boundary = self
            .boundary
            .hit(ray, &(f32::NEG_INFINITY..f32::INFINITY), rng)?
            .t;
        let next = start_boundary + 0.001;
        let end_boundary = self
            .boundary
            .hit(ray, &(next..f32::INFINITY), rng)?
            .t;

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

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.boundary.bounding_box(time_range)
    }
}

impl Hittable for Box<dyn Hittable + '_> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        self.as_ref().hit(ray, t_range, rng)
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.as_ref().bounding_box(time_range)
    }
}

impl<T: Hittable> Hittable for Vec<T> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let mut range = t_range.clone();
        let mut min_interaction: Option<HitInteraction> = None;
        for hittable in self {
            if let Some(hi) = hittable.hit(ray, &range, rng) {
                range.end = hi.t;
                min_interaction = Some(hi);
            }
        }
        min_interaction
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        // If any child is None -> None
        // Empty list -> None
        // else reduce(AABB::new_surounding)
        let mut result: Option<Aabb> = None;
        for hittable in self {
            if let Some(aabb) = &hittable.bounding_box(time_range) {
                if let Some(old) = &result {
                    result = Some(Aabb::new_surrounding_boxes(&[*old, *aabb]))
                } else {
                    result = Some(*aabb);
                }
            } else {
                return None;
            }
        }
        result
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct BoundingVolumeNode {
    aabb: Aabb,
    axis_id: usize,
    left: usize,
    right: usize,
}
pub struct BoundingVolumeHierarchy<T: Hittable> {
    items: Vec<T>,
    nodes: Vec<BoundingVolumeNode>,
    initial_index: usize,
}

impl<T: Hittable> BoundingVolumeHierarchy<T> {
    pub fn new(items: Vec<T>, time_range: &Range<f32>) -> Self {
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
}

impl<T: Hittable> Hittable for BoundingVolumeHierarchy<T> {
    fn hit(
        &self,
        ray: &Ray,
        t_range: &Range<f32>,
        rng: &mut common::TRng,
    ) -> Option<HitInteraction> {
        let mut t_range = t_range.clone();
        self._hit(self.initial_index, ray, &mut t_range, rng)
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        if self.initial_index < self.nodes.len() {
            Some(self.nodes[self.initial_index].aabb)
        } else {
            None
        }
    }
}
