use rand_distr::Distribution;

use crate::{
    ray::Ray,
    vec2::Vec2f,
    vec3::{Dir3, Point3},
};

pub struct Camera {
    position: Point3,
    upper_left_corner: Dir3,
    unit_right: Dir3,
    unit_up: Dir3,
    scaled_right: Dir3,
    scaled_up: Dir3,
    lens_radius: f32,
}

impl Camera {
    pub fn new_look_at(
        viewport_width: f32,
        viewport_height: f32,
        position: Point3,
        up: Dir3,
        look_at: Point3,
        aperture: f32,
        focus_offset: f32,
    ) -> Camera {
        let forward = look_at - position;
        Self::new(
            viewport_width,
            viewport_height,
            forward.length() + focus_offset,
            position,
            up,
            forward,
            aperture,
        )
    }
    pub fn new(
        viewport_width: f32,
        viewport_height: f32,
        focus_distance: f32,
        position: Point3,
        up: Dir3,
        forward: Dir3,
        aperture: f32,
    ) -> Camera {
        let unit_right = Dir3::cross(forward, up).unit();
        let unit_up = Dir3::cross(unit_right, forward).unit();
        let sforward = forward.with_length(focus_distance);

        Self {
            position,
            upper_left_corner: focus_distance
                * (unit_right * (viewport_width * -0.5) + unit_up * (viewport_height * 0.5))
                + sforward,
            unit_right,
            unit_up,
            scaled_right: unit_right * (focus_distance * viewport_width),
            scaled_up: unit_up * (focus_distance * viewport_height),
            lens_radius: aperture / 2.0,
        }
    }

    pub fn ray<TRng: rand::Rng>(&self, rng: &mut TRng, point: Vec2f) -> Ray {
        let [rdx, rdy]: [f32; 2] = rand_distr::UnitDisc.sample(rng);
        let offset = self.lens_radius * (rdx * self.unit_right + rdy * self.unit_up);

        Ray::new(
            self.position + offset,
            (self.upper_left_corner + point.x * self.scaled_right
                - point.y * self.scaled_up
                - offset)
                .unit(),
        )
    }
}
