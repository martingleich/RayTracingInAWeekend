use std::ops::RangeInclusive;

use rand_distr::Distribution;

use crate::{
    ray::Ray,
    vec2::Vec2f,
    vec3::{Dir3, Point3},
};

struct ActualCameraBuilder {
    viewport_width: f32,
    viewport_height: f32,
    position: Point3,
    up: Dir3,
    forward: Dir3,

    aperture: f32,
    focus_distance: f32,

    time_interval: RangeInclusive<f32>,
    shutter_pace: Vec2f,
}

pub trait CameraBuilderState {}
pub enum Initial {}
pub enum WaitForPosition {}
pub enum WaitForOrientation {}
pub enum Completed {}
impl CameraBuilderState for Initial {}
impl CameraBuilderState for WaitForPosition {}
impl CameraBuilderState for WaitForOrientation {}
impl CameraBuilderState for Completed {}

pub struct CameraBuilder<S: CameraBuilderState> {
    data: Box<ActualCameraBuilder>,
    phantom: std::marker::PhantomData<S>,
}

impl<S: CameraBuilderState> CameraBuilder<S> {
    fn new() -> Self {
        CameraBuilder::inner_new(Box::new(ActualCameraBuilder {
            viewport_width: 0.0,
            viewport_height: 0.0,
            aperture: 0.0,
            focus_distance: 1.0,
            forward: Dir3::FORWARD,
            up: Dir3::UP,
            position: Point3::ORIGIN,
            shutter_pace: Vec2f::ZERO,
            time_interval: 0.0..=0.0,
        }))
    }
    fn inner_new(data: Box<ActualCameraBuilder>) -> CameraBuilder<S> {
        CameraBuilder::<S> {
            data,
            phantom: std::marker::PhantomData {},
        }
    }
}

impl CameraBuilder<Initial> {
    pub fn viewport(
        mut self,
        viewport_width: f32,
        viewport_height: f32,
    ) -> CameraBuilder<WaitForPosition> {
        self.data.viewport_width = viewport_width;
        self.data.viewport_height = viewport_height;
        CameraBuilder::inner_new(self.data)
    }
    pub fn vertical_fov(
        self,
        vertical_field_of_view: f32,
        aspect_ratio: f32,
    ) -> CameraBuilder<WaitForPosition> {
        let h = (vertical_field_of_view.to_radians() * 0.5).tan();
        self.viewport(2.0 * h, 2.0 * h * aspect_ratio)
    }
}

impl CameraBuilder<WaitForPosition> {
    pub fn position(mut self, pos: Point3) -> CameraBuilder<WaitForOrientation> {
        self.data.position = pos;
        CameraBuilder::inner_new(self.data)
    }
}

impl CameraBuilder<WaitForOrientation> {
    pub fn orientation(mut self, up: Dir3, forward: Dir3) -> CameraBuilder<Completed> {
        self.data.up = up;
        self.data.forward = forward;
        CameraBuilder::inner_new(self.data)
    }
    pub fn look_at(mut self, up: Dir3, pos: Point3) -> CameraBuilder<Completed> {
        self.data.up = up;
        self.data.forward = pos - self.data.position;
        CameraBuilder::inner_new(self.data)
    }
    pub fn look_at_focus(mut self, up: Dir3, pos: Point3) -> CameraBuilder<Completed> {
        self.data.up = up;
        self.data.forward = pos - self.data.position;
        self.data.focus_distance = self.data.forward.length();
        CameraBuilder::inner_new(self.data)
    }
}

impl CameraBuilder<Completed> {
    pub fn focus_point(mut self, pos: Point3) -> Self {
        self.data.focus_distance = (self.data.position - pos).length();
        self
    }
    pub fn aperture(mut self, aperture: f32) -> Self {
        self.data.aperture = aperture;
        self
    }
    pub fn focus_distance(mut self, distance: f32) -> Self {
        self.data.focus_distance = distance;
        self
    }

    pub fn motion_blur(mut self, start: f32, end: f32) -> Self {
        self.data.time_interval = start..=end;
        self
    }
    pub fn build(self) -> Camera {
        self.data.build()
    }
}

impl ActualCameraBuilder {
    fn build(self) -> Camera {
        let unit_right = Dir3::cross(self.forward, self.up).unit();
        let unit_up = Dir3::cross(unit_right, self.forward).unit();
        let sforward = self.forward.with_length(self.focus_distance);

        let upper_left_corner = self.focus_distance
            * (unit_right * (self.viewport_width * -0.5) + unit_up * (self.viewport_height * 0.5))
            + sforward;
        Camera {
            position: self.position,
            upper_left_corner,
            unit_right,
            unit_up,
            scaled_right: unit_right * (self.focus_distance * self.viewport_width),
            scaled_up: unit_up * (self.focus_distance * self.viewport_height),
            lens_radius: self.aperture / 2.0,
            time_interval: self.time_interval,
            shutter_pace: self.shutter_pace,
        }
    }
}

pub struct Camera {
    position: Point3,
    upper_left_corner: Dir3,
    unit_right: Dir3,
    unit_up: Dir3,
    scaled_right: Dir3,
    scaled_up: Dir3,
    lens_radius: f32,
    time_interval: RangeInclusive<f32>,
    shutter_pace: Vec2f,
}

impl Camera {
    pub fn build() -> CameraBuilder<Initial> {
        CameraBuilder::<Initial>::new()
    }

    pub fn ray<TRng: rand::Rng>(&self, rng: &mut TRng, point: Vec2f) -> Ray {
        // Defocus blur
        let offset = if self.lens_radius > 0.0 {
            let [rdx, rdy]: [f32; 2] = rand_distr::UnitDisc.sample(rng);
            self.lens_radius * (rdx * self.unit_right + rdy * self.unit_up)
        } else {
            Dir3::ZERO
        };

        // Motion blur
        let time = rng.gen_range(self.time_interval.clone()) + Vec2f::dot(self.shutter_pace, point);

        Ray::new(
            self.position + offset,
            (self.upper_left_corner + point.x * self.scaled_right
                - point.y * self.scaled_up
                - offset)
                .unit(),
            time,
        )
    }
}
