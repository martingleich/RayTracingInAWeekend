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
}

impl Material {
    pub fn scatter<TRng: Rng>(
        &self,
        ray: &Ray,
        interaction: &HitInteraction,
        rng: &mut TRng,
    ) -> Option<(Color, Ray)> {
        match self {
            Material::Lambert { albedo } => {
                let direction = (interaction.normal + Dir3::new_from_arr(UnitSphere.sample(rng)))
                    .unit_or_else(interaction.normal);
                let scattered = Ray::new(interaction.position, direction);
                Some((*albedo, scattered))
            }
            Material::Metal { albedo, fuzz } => {
                let fuzz_dir = if *fuzz > 0.0 {
                    *fuzz * Dir3::new_from_arr(UnitBall.sample(rng))
                } else {
                    Dir3::ZERO
                };
                let direction = Dir3::reflect(ray.direction, interaction.normal) + fuzz_dir;
                if Dir3::dot(direction, interaction.normal) > 0.0 {
                    let scattered = Ray::new(interaction.position, direction.unit());
                    Some((*albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}
