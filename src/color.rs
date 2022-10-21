use crate::math;
use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    v: Vec3<f32>,
}

impl Color {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            v: Vec3::new(r, g, b),
        }
    }

    pub fn to_rgb8(self) -> [u8; 3] {
        let ir = math::clamp(0.0, 255.0, self.v.e[0] * 256.0) as u8;
        let ig = math::clamp(0.0, 255.0, self.v.e[1] * 256.0) as u8;
        let ib = math::clamp(0.0, 255.0, self.v.e[2] * 256.0) as u8;
        [ir, ig, ib]
    }

    pub const BLACK: Self = Self {
        v: Vec3 { e: [0.0, 0.0, 0.0] },
    };

    pub const WHITE: Self = Self {
        v: Vec3 { e: [1.0, 1.0, 1.0] },
    };

    pub fn gamma2(&self) -> Self {
        Self::new_rgb(self.v.e[0].sqrt(), self.v.e[1].sqrt(), self.v.e[2].sqrt())
    }

    pub fn convolution(a: Self, b: Self) -> Self {
        Self::new_rgb(
            a.v.e[0] * b.v.e[0],
            a.v.e[1] * b.v.e[1],
            a.v.e[2] * b.v.e[2],
        )
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Self::Output { v: self.v + rhs.v }
    }
}

impl std::ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { v: self.v * rhs }
    }
}

impl std::ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { v: self.v / rhs }
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Color::BLACK;
        for x in iter {
            result += x;
        }
        result
    }
}
