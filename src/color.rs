use crate::math;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new_rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn to_ppm_string(&self) -> String {
        let ir = math::clamp(0.0, 255.0, self.r * 256.0) as u8;
        let ig = math::clamp(0.0, 255.0, self.g * 256.0) as u8;
        let ib = math::clamp(0.0, 255.0, self.b * 256.0) as u8;
        format!("{ir} {ig} {ib}")
    }

    pub const BLACK: Self = Self {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };

    pub fn gamma2(&self) -> Self {
        Self::new_rgb(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl std::ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
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
        self * (1.0 / rhs)
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Color::BLACK;
        for x in iter {
            result += x;
        }
        return result;
    }
}
