use crate::vec3::{Dir3, Point3, Vec3};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Transformation {
    offset: Dir3,
    y_sine: f32,
    y_cosine: f32,
}

impl Transformation {
    pub const ZERO: Transformation = Transformation {
        offset: Dir3::ZERO,
        y_sine: 0.0,
        y_cosine: 1.0,
    };
    pub fn translate_xyz(&self, x: f32, y: f32, z: f32) -> Self {
        self.translate(Dir3::new(x, y, z))
    }
    pub fn translate(&self, offset: Dir3) -> Self {
        let mut result = *self;
        result.offset += offset;
        result
    }
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }
    pub fn rotate_around_up(&self, angle: f32) -> Self {
        let mut result = *self;
        let (s, c) = angle.to_radians().sin_cos();
        rotate_around_up(c, s, &mut result.offset.0);
        let mut r = Vec3::new(self.y_sine, 0.0, self.y_cosine);
        rotate_around_up(c, s, &mut r);
        result.y_sine = r.e[0];
        result.y_cosine = r.e[2];
        result
    }
    pub fn apply_point_mut(&self, point: &mut Point3) {
        rotate_around_up(self.y_cosine, self.y_sine, &mut point.0);
        *point += self.offset;
    }

    pub fn apply_normal_mut(&self, dir: &mut Dir3) {
        rotate_around_up(self.y_cosine, self.y_sine, &mut dir.0);
    }
    pub fn apply_direction_mut(&self, dir: &mut Dir3) {
        rotate_around_up(self.y_cosine, self.y_sine, &mut dir.0);
    }

    pub fn reverse_point_mut(&self, point: &mut Point3) {
        *point -= self.offset;
        rotate_around_up(self.y_cosine, -self.y_sine, &mut point.0);
    }

    pub fn reverse_normal_mut(&self, dir: &mut Dir3) {
        rotate_around_up(self.y_cosine, -self.y_sine, &mut dir.0);
    }

    pub fn apply_point(&self, mut point: Point3) -> Point3 {
        self.apply_point_mut(&mut point);
        point
    }
    pub fn apply_normal(&self, mut dir: Dir3) -> Dir3 {
        self.apply_normal_mut(&mut dir);
        dir
    }
    pub fn split_translation_remainder(&self) -> (Dir3, Transformation) {
        let mut remainder = *self;
        remainder.offset = Dir3::ZERO;
        (self.offset, remainder)
    }
    pub fn apply_direction(&self, mut dir: Dir3) -> Dir3 {
        self.apply_direction_mut(&mut dir);
        dir
    }
    pub fn reverse_point(&self, mut point: Point3) -> Point3 {
        self.reverse_point_mut(&mut point);
        point
    }
    pub fn reverse_normal(&self, mut dir: Dir3) -> Dir3 {
        self.reverse_normal_mut(&mut dir);
        dir
    }
    pub fn apply_distance(&self, distance: f32) -> f32 {
        distance
    }
    pub fn reverse_distance(&self, distance: f32) -> f32 {
        distance
    }
    pub fn then(&self, next: &Transformation) -> Transformation {
        let offset = next.apply_direction(self.offset) + next.offset;
        let y_sine = self.y_sine * next.y_cosine + self.y_cosine * next.y_sine;
        let y_cosine = self.y_cosine * next.y_cosine - self.y_sine * next.y_sine;
        Transformation {
            offset,
            y_sine,
            y_cosine,
        }
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Self::ZERO
    }
}

fn rotate_around_up(c: f32, s: f32, v: &mut Vec3<f32>) {
    let (x, y) = (v.e[0], v.e[2]);
    v.e[0] = c * x + s * y;
    v.e[2] = -s * x + c * y;
}
