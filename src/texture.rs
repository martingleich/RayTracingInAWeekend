use crate::{color::Color, vec2::Vec2f, hittable::HitInteraction};

#[derive(Debug, PartialEq, Clone)]
pub enum Texture<'a> {
    Solid {
        color: Color,
    },
    Checker {
        frequency: f32,
        even: &'a Texture<'a>,
        odd: &'a Texture<'a>,
    },
}

impl<'a> Texture<'a> {
    pub fn sample(&self, interaction: &HitInteraction) -> Color {
        match self {
            Texture::Solid { color } => *color,
            Texture::Checker { frequency, even, odd } => 
            {
                let s = interaction.position.0 * *frequency;
                let sines = s.e[0].sin() * s.e[1].sin() * s.e[2].sin();
                let t = if sines < 0.0 {even} else {odd};
                t.sample(interaction)
            },
        }
    }
}
