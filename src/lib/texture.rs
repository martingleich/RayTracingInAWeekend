use crate::{color::Color, hittable::HitInteraction, perlin::Perlin, vec3::Point3};

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
    Marble {
        scale: f32,
        noise: Perlin,
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
            Texture::Marble { scale, noise } => {
                Color::new_rgb(1.0, 1.0, 1.0)
                    * 0.5
                    * (1.0
                        + (interaction.position.0.e[2] * *scale
                            + 10.0 * noise.turbulence(interaction.position, 7, 0.5))
                        .sin())
            }
        }
    }
}
