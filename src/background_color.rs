use crate::{color::Color, math, ray::Ray, vec3::Dir3};

pub enum BackgroundColor {
    Sky,
    Solid { color: Color },
}

impl BackgroundColor {
    pub fn sample(&self, ray: &Ray) -> Color {
        match *self {
            BackgroundColor::Sky => {
                let t = 0.5 * (Dir3::dot(Dir3::UP, ray.direction) + 1.0);
                let ground_color = Color::new_rgb(0.5, 0.7, 1.0);
                let sky_color = Color::new_rgb(1.0, 1.0, 1.0);
                math::lerp(sky_color, ground_color, t)
            }
            BackgroundColor::Solid { color } => color,
        }
    }
}
