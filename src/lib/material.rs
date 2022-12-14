use std::f32::consts::PI;

use crate::common;
use crate::hittable::HitInteraction;
use crate::{color::Color, texture::Texture};

use crate::ray::Ray;
use crate::vec3::Dir3;
use rand::Rng;
use rand_distr::{Distribution, UnitBall, UnitSphere};

pub enum MaterialScatteringDistribution {
    Cosine(Dir3),
    Mirror(Dir3),
}

impl MaterialScatteringDistribution {
    pub fn generate(&self, rng: &mut common::TRng) -> Dir3 {
        match *self {
            MaterialScatteringDistribution::Cosine(normal) => {
                (normal + Dir3::new_from_arr(UnitSphere.sample(rng))).unit_or_else(normal)
            }
            MaterialScatteringDistribution::Mirror(direction) => direction,
        }
    }
    pub fn value(&self, direction: Dir3) -> f32 {
        match *self {
            MaterialScatteringDistribution::Cosine(normal) => {
                Dir3::dot(normal, direction).max(0.0) / PI
            }
            MaterialScatteringDistribution::Mirror(_) => f32::INFINITY,
        }
    }
    pub fn is_discrete(&self) -> bool {
        match *self {
            MaterialScatteringDistribution::Cosine(_) => false,
            MaterialScatteringDistribution::Mirror(_) => true,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Material<'a> {
    Lambert { albedo: &'a Texture<'a> },
    Metal { albedo: &'a Texture<'a>, fuzz: f32 },
    Dielectric { index_of_refraction: f32 },
    DiffuseLight { emit: &'a Texture<'a> },
    Isotropic { albedo: &'a Texture<'a> },
}

impl<'a> Material<'a> {
    pub fn scatter(
        &self,
        ray: &Ray,
        interaction: &HitInteraction,
        rng: &mut common::TRng,
    ) -> Option<(Color, MaterialScatteringDistribution)> {
        match *self {
            Material::Lambert { albedo } => {
                let color = albedo.sample(interaction);
                let pdf = MaterialScatteringDistribution::Cosine(interaction.normal);
                Some((color, pdf))
            }

            Material::Metal { albedo, fuzz } => {
                let fuzz_dir = if fuzz > 0.0 {
                    fuzz * Dir3::new_from_arr(UnitBall.sample(rng))
                } else {
                    Dir3::ZERO
                };
                let direction = Dir3::reflect(ray.direction, interaction.normal) + fuzz_dir;
                if Dir3::dot(direction, interaction.normal) > 0.0 {
                    let scattered = MaterialScatteringDistribution::Mirror(direction.unit());
                    let color = albedo.sample(interaction);
                    Some((color, scattered))
                } else {
                    None
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
                    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
                    let rs = r0 * r0;
                    rs + (1.0 - rs) * (1.0 - cosine).powi(5)
                }
                let refraction_ratio = if interaction.front_face {
                    1.0 / index_of_refraction
                } else {
                    index_of_refraction
                };
                let cos_theta = f32::min(Dir3::dot(-ray.direction, interaction.normal), 1.0);
                let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);
                let cannot_refract = refraction_ratio * sin_theta > 1.0;
                let direction = if cannot_refract
                    || (reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>())
                {
                    Dir3::reflect(ray.direction, interaction.normal)
                } else {
                    Dir3::refract(ray.direction, interaction.normal, refraction_ratio)
                };
                let scattered = MaterialScatteringDistribution::Mirror(direction.unit());
                Some((Color::WHITE, scattered))
            }
            Material::Isotropic { albedo } => {
                let scattered = MaterialScatteringDistribution::Mirror(Dir3::new_from_arr(
                    UnitSphere.sample(rng),
                ));
                Some((albedo.sample(interaction), scattered))
            }
            _ => None,
        }
    }

    pub fn scattering_pdf(
        &self,
        _ray_in: &Ray,
        ray_scattered: &Ray,
        interaction: &HitInteraction,
    ) -> f32 {
        match *self {
            Material::Lambert { albedo: _ } => {
                let cosine = Dir3::dot(interaction.normal, ray_scattered.direction);
                let clamped_cosine = cosine.max(0.0);
                clamped_cosine / PI
            }
            _ => 0.0,
        }
    }

    pub fn emit(&self, interaction: &HitInteraction) -> Color {
        match *self {
            Material::DiffuseLight { emit } => emit.sample(interaction),
            _ => Color::BLACK,
        }
    }
}
