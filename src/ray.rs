use crate::vec3::Dir3;
use crate::vec3::Point3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Dir3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Dir3) -> Self {
        debug_assert!(direction.length_squared() > 0.99 && direction.length_squared() < 1.01);
        Self { origin, direction }
    }

    pub fn at(self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}
