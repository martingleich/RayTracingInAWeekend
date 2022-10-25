use std::ops::Range;

use crate::{
    aabb::Aabb,
    ray::Ray,
    vec2::Vec2f,
    vec3::{Dir3, Point3},
    Material,
};

#[derive(Debug, Clone, PartialEq)]
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
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction>;
    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb>;
}

#[derive(Debug, Clone, PartialEq)]
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
    let theta = pos.0.e[1].acos();
    let phi = f32::atan2(-pos.0.e[2], pos.0.e[0]) + std::f32::consts::PI;
    Vec2f::new(phi / std::f32::consts::TAU, theta / std::f32::consts::PI)
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
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

pub struct Rect<'a> {
    corner: Point3,
    axis_right: Dir3,
    axis_up: Dir3,
    material: &'a Material<'a>,
}

impl<'a> Rect<'a> {
    pub fn new_xy(center: Point3, width: f32, height: f32, material: &'a Material<'a>) -> Self {
        let axis_right = Dir3::RIGHT * width;
        let axis_up = Dir3::UP * height;
        Self {
            corner: center - 0.5 * (axis_right + axis_up),
            axis_right,
            axis_up,
            material,
        }
    }
    pub fn new_xz(center: Point3, width: f32, depth: f32, material: &'a Material<'a>) -> Self {
        let axis_right = Dir3::RIGHT * width;
        let axis_up = Dir3::FORWARD * depth;
        Self {
            corner: center - 0.5 * (axis_right + axis_up),
            axis_right,
            axis_up,
            material,
        }
    }
    pub fn new_yz(center: Point3, height: f32, depth: f32, material: &'a Material<'a>) -> Self {
        let axis_right = Dir3::UP * height;
        let axis_up = Dir3::FORWARD * depth;
        Self {
            corner: center - 0.5 * (axis_right + axis_up),
            axis_right,
            axis_up,
            material,
        }
    }
}

impl<'a> Hittable for Rect<'a> {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        let normal = Dir3::cross(self.axis_right, self.axis_up).unit();
        let denom = Dir3::dot(ray.direction, normal);
        if denom.abs() > 0.0001 {
            let t = Dir3::dot(normal, self.corner - ray.origin) / denom;
            if t_range.contains(&t) {
                let position = ray.at(t);
                let q = position - self.corner;
                let v_temp = Dir3::cross(normal, self.axis_up);
                let x = Dir3::dot(q, v_temp) / Dir3::dot(self.axis_right, v_temp);
                if (0.0..1.0).contains(&x) {
                    let v_temp = Dir3::cross(normal, self.axis_right);
                    let y = Dir3::dot(q, v_temp) / Dir3::dot(self.axis_up, v_temp);
                    if (0.0..1.0).contains(&y) {
                        let uv = Vec2f::new(x, y);
                        return Some(HitInteraction::new_from_ray(ray, &position, &normal, t, self.material, uv));
                    }
                }
            };
        };
        None
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        let p1 = self.corner;
        let p2 = self.corner + self.axis_right;
        let p3 = self.corner + self.axis_up;
        let p4 = self.corner + self.axis_right + self.axis_up;
        Some(Aabb::new_surrounding_points(&[p1, p2, p3, p4]))
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
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        // Instead of transforming the object the just move the ray backward
        let mut moved_ray = *ray;
        moved_ray.origin = moved_ray.origin - self.velocity * ray.time;
        self.hittable.hit(&moved_ray, t_range).map(|mut f|
        {
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
        Some(Aabb::new_surrounding_boxes(&start_box, &end_box))
    }
}

pub struct TranslatedHittable<'a, T: Hittable> {
    offset: Dir3,
    hittable: &'a T,
}

impl<'a, T: Hittable> TranslatedHittable<'a, T> {
    pub fn new(hittable: &'a T, offset: Dir3) -> Self {
        Self { hittable, offset }
    }
}

impl<'a, T: Hittable> Hittable for TranslatedHittable<'a, T> {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        // Instead of transforming the object the just move the ray backward
        let mut moved_ray = *ray;
        moved_ray.origin = moved_ray.origin - self.offset;
        self.hittable.hit(&moved_ray, t_range).map(|mut f| {
            f.position += self.offset;
            f
        })
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.hittable.bounding_box(time_range).map(|f| {f.translate(self.offset)})
    }
}

pub struct Box<'a> {
    aabb : Aabb,
    sides : HittableList<Rect<'a>>,
    material : &'a Material<'a>
}

impl<'a> Box<'a> {
    pub fn new(aabb: Aabb, material : &'a Material<'a>) -> Self {
        let sides = {
            let mut sides = HittableList::new();
            sides.push(hittable)
            sides
        }
        Self { aabb, sides }
    }
}

pub struct HittableList<T: Hittable> {
    hittables: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self {
            hittables: Vec::new(),
        }
    }
    pub fn push(&mut self, hittable: T) {
        self.hittables.push(hittable);
    }
}

impl Hittable for Box<dyn Hittable + '_> {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        self.as_ref().hit(ray, t_range)
    }

    fn bounding_box(&self, time_range: &Range<f32>) -> Option<Aabb> {
        self.as_ref().bounding_box(time_range)
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        let mut range = t_range.clone();
        let mut min_interaction: Option<HitInteraction> = None;
        for hittable in &self.hittables {
            if let Some(hi) = hittable.hit(ray, &range) {
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
        for hittable in &self.hittables {
            if let Some(aabb) = &hittable.bounding_box(time_range) {
                if let Some(old) = &result {
                    result = Some(Aabb::new_surrounding_boxes(old, aabb))
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

struct BoundingVolumeHierarchy {
    left: Box<dyn Hittable>,
    right: Option<Box<dyn Hittable>>,
    aabb: Aabb,
}

impl BoundingVolumeHierarchy {
    pub fn new(
        hittables: Vec<Box<dyn Hittable>>,
        time_range: &Range<f32>,
    ) -> BoundingVolumeHierarchy {
        let hittables = hittables
            .into_iter()
            .map(|h| {
                let aabb = h.bounding_box(time_range).unwrap();
                (h, aabb)
            })
            .collect::<Vec<_>>();
        Self::new_inner(hittables, 0)
    }

    fn new_inner(
        mut hittables: Vec<(Box<dyn Hittable>, Aabb)>,
        axis_id: usize,
    ) -> BoundingVolumeHierarchy {
        if hittables.len() == 1 {
            let (hittable, aabb) = hittables.pop().unwrap();
            Self {
                aabb,
                left: hittable,
                right: None,
            }
        } else {
            let comparer =
                |a: &Aabb, b: &Aabb| a.min.0.e[axis_id].partial_cmp(&b.min.0.e[axis_id]).unwrap();
            hittables.sort_by(|a, b| comparer(&a.1, &b.1));
            if hittables.len() == 2 {
                let (left_hittable, left_box) = hittables.pop().unwrap();
                let (right_hittable, right_box) = hittables.pop().unwrap();
                Self {
                    aabb: Aabb::new_surrounding_boxes(&left_box, &right_box),
                    left: left_hittable,
                    right: Some(right_hittable),
                }
            } else {
                let mid = hittables.len() / 2;
                let mut right_half = Vec::new();
                for _ in 0..mid {
                    right_half.push(hittables.pop().unwrap());
                }
                let left = Box::new(Self::new_inner(hittables, (axis_id + 1) % 3));
                let right = Box::new(Self::new_inner(right_half, (axis_id + 1) % 3));
                Self {
                    aabb: Aabb::new_surrounding_boxes(&left.aabb, &right.aabb),
                    left,
                    right: Some(right),
                }
            }
        }
    }
}

impl Hittable for BoundingVolumeHierarchy {
    fn hit(&self, ray: &Ray, t_range: &Range<f32>) -> Option<HitInteraction> {
        if self.aabb.hit(ray, t_range) {
            let mut range = t_range.clone();
            let mut min_interaction: Option<HitInteraction> = None;
            if let Some(hi) = self.left.hit(ray, &range) {
                range.end = hi.t;
                min_interaction = Some(hi);
            }
            if let Some(right) = &self.right {
                if let Some(hi) = right.hit(ray, &range) {
                    range.end = hi.t;
                    min_interaction = Some(hi);
                }
            }
            min_interaction
        } else {
            None
        }
    }

    fn bounding_box(&self, _time_range: &Range<f32>) -> Option<Aabb> {
        Some(self.aabb)
    }
}
