use crate::{
    ray::Ray,
    vec2::Vec2f,
    vec3::{Dir3, Point3},
};

pub struct Camera {
    position: Point3,
    upper_left_corner: Dir3,
    right: Dir3,
    up: Dir3,
}

impl Camera {
    pub fn new_look_at(
        viewport_width: f32,
        viewport_height: f32,
        focal_length: f32,
        position: Point3,
        up: Dir3,
        look_at: Point3,
    ) -> Camera {
        Self::new(
            viewport_width,
            viewport_height,
            focal_length,
            position,
            up,
            look_at - position,
        )
    }
    pub fn new(
        viewport_width: f32,
        viewport_height: f32,
        focal_length: f32,
        position: Point3,
        up: Dir3,
        forward: Dir3,
    ) -> Camera {
        let sright = Dir3::cross(forward, up).with_length(viewport_width);
        let sup = Dir3::cross(sright, forward).with_length(viewport_height);
        let sforward = forward.with_length(focal_length);

        Self {
            position,
            upper_left_corner: sright * -0.5 + sup * 0.5 + sforward,
            right: sright,
            up: sup,
        }
    }

    pub fn ray(&self, point: Vec2f) -> Ray {
        Ray::new(
            self.position,
            (self.upper_left_corner + point.x * self.right - point.y * self.up).unit(),
        )
    }
}
