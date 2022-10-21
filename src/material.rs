use crate::color::Color;
use crate::hittable::HitInteraction;

use crate::ray::Ray;
use crate::vec3::Dir3;
use rand::Rng;
use rand_distr::{Distribution, UnitBall, UnitSphere};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Material {
    Lambert { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { index_of_refraction: f32 },
}

impl Material {
    pub fn scatter<TRng: Rng>(
        &self,
        ray: &Ray,
        interaction: &HitInteraction,
        rng: &mut TRng,
    ) -> Option<(Color, Ray)> {
        match *self {
            Material::Lambert { albedo } => {
                let direction = (interaction.normal + Dir3::new_from_arr(UnitSphere.sample(rng)))
                    .unit_or_else(interaction.normal);
                let scattered = Ray::new(interaction.position, direction);
                Some((albedo, scattered))
            }
            Material::Metal { albedo, fuzz } => {
                let fuzz_dir = if fuzz > 0.0 {
                    fuzz * Dir3::new_from_arr(UnitBall.sample(rng))
                } else {
                    Dir3::ZERO
                };
                let direction = Dir3::reflect(ray.direction, interaction.normal) + fuzz_dir;
                if Dir3::dot(direction, interaction.normal) > 0.0 {
                    let scattered = Ray::new(interaction.position, direction.unit());
                    Some((albedo, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let refraction_ratio = if interaction.front_face {
                    1.0 / index_of_refraction
                } else {
                    index_of_refraction
                };
                let cos_theta = f32::min(Dir3::dot(-ray.direction, interaction.normal), 1.0);
                let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || (Self::reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>())
                {
                    Dir3::reflect(ray.direction, interaction.normal)
                } else {
                    Dir3::refract(ray.direction, interaction.normal, refraction_ratio)
                };
                let scattered = Ray::new(interaction.position, direction);
                Some((Color::WHITE, scattered))
            }
        }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let rs = r0 * r0;
        rs + (1.0 - rs) * (1.0 - cosine).powi(5)
    }
}
