use crate::hittable::rect_geometry::RectGeometry;
use crate::{common, Dir3, Point3};

pub enum WorldScatteringDistributionProvider {
    Rect(RectGeometry),
    //Sphere(SphereGeometry),
    //AxisAlignedBox(Aabb),
    //List(Vec<Box<WorldScatteringDistributionProvider>>)
}

pub struct WorldScatteringDistribution<'a> {
    provider: &'a WorldScatteringDistributionProvider,
    origin: Point3,
}

impl WorldScatteringDistributionProvider {
    pub fn generate(&self, origin: &Point3) -> Option<WorldScatteringDistribution> {
        match self {
            _ => Some(WorldScatteringDistribution {
                provider: self,
                origin: *origin,
            }),
        }
    }
}

impl<'a> WorldScatteringDistribution<'a> {
    pub fn generate(&self, rng: &mut common::TRng) -> Dir3 {
        match self.provider {
            WorldScatteringDistributionProvider::Rect(geo) => geo.generate(self.origin, rng),
        }
    }
    pub fn value(&self, direction: Dir3) -> f32 {
        match self.provider {
            WorldScatteringDistributionProvider::Rect(geo) => geo.value(self.origin, direction),
        }
    }
}
