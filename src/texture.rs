use crate::{color::Color, hittable::HitInteraction, vec3::Point3, perlin::Perlin};

#[derive(Debug, Clone)]
pub enum Texture<'a> {
    Solid {
        color: Color,
    },
    Checker {
        inv_frequency: f32,
        even: &'a Texture<'a>,
        odd: &'a Texture<'a>,
    },
    Noise {
        noise : Perlin,
    },
    Image {
        image: &'a image::RgbImage,
    },
}

impl<'a> Texture<'a> {
    pub fn sample(&self, interaction: &HitInteraction) -> Color {
        match self {
            Texture::Solid { color } => *color,
            Texture::Checker {
                inv_frequency: frequency,
                even,
                odd,
            } => {
                let s = (interaction.position - Point3::ORIGIN) * *frequency;
                let sines = s.right().sin() * s.up().sin() * s.forward().sin();
                let t = if sines < 0.0 { even } else { odd };
                t.sample(interaction)
            }
            Texture::Image { image } => {
                let pix_u =
                    ((interaction.uv.x * image.width() as f32) as u32).min(image.width() - 1);
                let pix_v =
                    ((interaction.uv.y * image.height() as f32) as u32).min(image.height() - 1);

                Color::new_rgb8(image.get_pixel(pix_u, pix_v).0)
            }
            Texture::Noise { noise } => {
                noise.sample(interaction.position) * Color::new_rgb(1.0, 1.0, 1.0)
            },
        }
    }
}
