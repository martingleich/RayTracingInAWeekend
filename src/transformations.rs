use crate::vec3::{Dir3, Point3, Vec3};

pub trait Transformation: Send + Sync {
    fn apply_point_mut(&self, point: &mut Point3);
    fn apply_dir_mut(&self, dir: &mut Dir3);

    fn reverse_point_mut(&self, point: &mut Point3);
    fn reverse_dir_mut(&self, dir: &mut Dir3);

    fn apply_point(&self, mut point: Point3) -> Point3 {
        self.apply_point_mut(&mut point);
        point
    }
    fn apply_dir(&self, mut dir: Dir3) -> Dir3 {
        self.apply_dir_mut(&mut dir);
        dir
    }
    fn reverse_point(&self, mut point: Point3) -> Point3 {
        self.reverse_point_mut(&mut point);
        point
    }
    fn reverse_dir(&self, mut dir: Dir3) -> Dir3 {
        self.reverse_dir_mut(&mut dir);
        dir
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Translation {
    pub offset: Dir3,
}

impl Transformation for Translation {
    fn apply_point_mut(&self, point: &mut Point3) {
        *point += self.offset;
    }

    fn apply_dir_mut(&self, _dir: &mut Dir3) {}

    fn reverse_point_mut(&self, point: &mut Point3) {
        *point -= self.offset;
    }

    fn reverse_dir_mut(&self, _dir: &mut Dir3) {}
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct RotationAroundUp {
    pub sin_angle: f32,
    pub cos_angle: f32,
}

impl RotationAroundUp {
    pub fn new(degrees: f32) -> Self {
        let (sin_angle, cos_angle) = degrees.to_radians().sin_cos();
        Self {
            sin_angle,
            cos_angle,
        }
    }
}

fn rotate_around_up(rot: &RotationAroundUp, c: &mut Vec3<f32>) {
    c.e[0] = rot.cos_angle * c.e[0] + rot.sin_angle * c.e[2];
    c.e[2] = -rot.sin_angle * c.e[0] + rot.cos_angle * c.e[2];
}
fn inv_rotate_around_up(rot: &RotationAroundUp, c: &mut Vec3<f32>) {
    c.e[0] = rot.cos_angle * c.e[0] - rot.sin_angle * c.e[2];
    c.e[2] = rot.sin_angle * c.e[0] + rot.cos_angle * c.e[2];
}
impl Transformation for RotationAroundUp {
    fn apply_point_mut(&self, point: &mut Point3) {
        rotate_around_up(self, &mut point.0);
    }

    fn apply_dir_mut(&self, dir: &mut Dir3) {
        rotate_around_up(self, &mut dir.0);
    }

    fn reverse_point_mut(&self, point: &mut Point3) {
        inv_rotate_around_up(self, &mut point.0);
    }

    fn reverse_dir_mut(&self, dir: &mut Dir3) {
        inv_rotate_around_up(self, &mut dir.0);
    }
}
