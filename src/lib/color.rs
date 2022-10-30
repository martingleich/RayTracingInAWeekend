use crate::math;
use crate::vec3::Vec3;

#[derive(
    Debug,
    PartialEq,
    Clone,
    Copy,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Neg,
    derive_more::Mul,
    derive_more::MulAssign,
    derive_more::Div,
    derive_more::DivAssign,
    derive_more::Sum,
)]
pub struct Color(Vec3<f32>);

impl Color {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }
    pub fn new_rgb_arr(c: [f32; 3]) -> Self {
        Self::new_rgb(c[0], c[1], c[2])
    }
    pub fn new_rgb8(c: [u8; 3]) -> Self {
        Self(Vec3::new(
            c[0] as f32 / 255.0,
            c[1] as f32 / 255.0,
            c[2] as f32 / 255.0,
        ))
    }
    pub fn to_rgb8(self) -> [u8; 3] {
        let ir = math::clamp(0.0, 255.0, self.0.e[0] * 256.0) as u8;
        let ig = math::clamp(0.0, 255.0, self.0.e[1] * 256.0) as u8;
        let ib = math::clamp(0.0, 255.0, self.0.e[2] * 256.0) as u8;
        [ir, ig, ib]
    }

    pub const BLACK: Self = Self(Vec3 { e: [0.0, 0.0, 0.0] });

    pub const WHITE: Self = Self(Vec3 { e: [1.0, 1.0, 1.0] });

    pub fn gamma2(&self) -> Self {
        Self::new_rgb(self.0.e[0].sqrt(), self.0.e[1].sqrt(), self.0.e[2].sqrt())
    }

    pub fn convolution(a: Self, b: Self) -> Self {
        Self::new_rgb(
            a.0.e[0] * b.0.e[0],
            a.0.e[1] * b.0.e[1],
            a.0.e[2] * b.0.e[2],
        )
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}
