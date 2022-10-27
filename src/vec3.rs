#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Vec3<T> {
    pub e: [T; 3],
}

impl<T> Vec3<T> {
    pub fn new(e0: T, e1: T, e2: T) -> Self {
        Self { e: [e0, e1, e2] }
    }
}

impl<T: std::ops::Sub<T, Output = T> + Copy> std::ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Self::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl<T: std::ops::SubAssign<T> + Copy> std::ops::SubAssign<Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: Vec3<T>) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl<T: std::ops::Add<T, Output = T> + Copy> std::ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Self::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl<T: std::ops::AddAssign<T> + Copy> std::ops::AddAssign<Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl<T: std::ops::Neg<Output = T> + Copy> std::ops::Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl<T: std::ops::Mul<f32, Output = T> + Copy> std::ops::Mul<f32> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl<T: std::ops::MulAssign<f32> + Copy> std::ops::MulAssign<f32> for Vec3<T> {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl<T: std::ops::Div<f32, Output = T> + Copy> std::ops::Div<f32> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

impl<T: std::ops::AddAssign<T> + std::default::Default + Copy> std::iter::Sum for Vec3<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Default::default();
        for x in iter {
            result += x;
        }
        result
    }
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Point3(pub Vec3<f32>);

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
pub struct Dir3(pub Vec3<f32>);

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3 { e: [x, y, z] })
    }
    pub fn new_from_arr(c: [f32; 3]) -> Self {
        Self::new(c[0], c[1], c[2])
    }
    pub const ORIGIN: Self = Self(Vec3 { e: [0.0, 0.0, 0.0] });

    pub fn right(self) -> f32 {
        self.0.e[0]
    }
    pub fn up(self) -> f32 {
        self.0.e[1]
    }
    pub fn forward(self) -> f32 {
        self.0.e[2]
    }
    pub fn components(self) -> [f32; 3] {
        self.0.e
    }
}

impl Dir3 {
    pub fn new(right: f32, up: f32, forward: f32) -> Self {
        Self(Vec3 {
            e: [right, up, forward],
        })
    }
    pub fn new_from_arr(c: [f32; 3]) -> Self {
        Self::new(c[0], c[1], c[2])
    }

    pub fn right(self) -> f32 {
        self.0.e[0]
    }
    pub fn up(self) -> f32 {
        self.0.e[1]
    }
    pub fn forward(self) -> f32 {
        self.0.e[2]
    }

    pub const ZERO: Self = Self(Vec3 { e: [0.0, 0.0, 0.0] });
    pub const RIGHT: Self = Self(Vec3 { e: [1.0, 0.0, 0.0] });
    pub const LEFT: Self = Self(Vec3 {
        e: [-1.0, 0.0, 0.0],
    });
    pub const UP: Self = Self(Vec3 { e: [0.0, 1.0, 0.0] });
    pub const DOWN: Self = Self(Vec3 {
        e: [0.0, -1.0, 0.0],
    });
    pub const FORWARD: Self = Self(Vec3 {
        e: [0.0, 0.0, -1.0],
    });
    pub const BACKWARD: Self = Self(Vec3 { e: [0.0, 0.0, 1.0] });

    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f32 {
        Self::dot(self, self)
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.0.e[0] * b.0.e[0] + a.0.e[1] * b.0.e[1] + a.0.e[2] * b.0.e[2]
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Self(Vec3 {
            e: [
                a.0.e[1] * b.0.e[2] - a.0.e[2] * b.0.e[1],
                a.0.e[2] * b.0.e[0] - a.0.e[0] * b.0.e[2],
                a.0.e[0] * b.0.e[1] - a.0.e[1] * b.0.e[0],
            ],
        })
    }

    pub fn unit(self) -> Self {
        self.with_length(1.0)
    }
    pub fn with_length(self, length: f32) -> Self {
        self * (length / self.length())
    }

    pub fn near_zero(self) -> bool {
        let eps: f32 = 1e-8;
        self.0.e[0].abs() < eps && self.0.e[1].abs() < eps && self.0.e[2].abs() < eps
    }
    pub fn near_zero_or_else(self, default: Dir3) -> Dir3 {
        if self.near_zero() {
            default
        } else {
            self
        }
    }
    pub fn unit_or_else(self, default: Dir3) -> Dir3 {
        let len_sq = self.length_squared();
        if len_sq > 1e-8 {
            self / len_sq.sqrt()
        } else {
            default
        }
    }

    pub fn reflect(ray: Dir3, normal: Dir3) -> Dir3 {
        ray - (2.0 * Self::dot(ray, normal)) * normal
    }
    pub fn refract(ray: Dir3, normal: Dir3, etai_over_etat: f32) -> Dir3 {
        let cos_theta = f32::min(Self::dot(-ray, normal), 1.0);
        let r_out_perp = etai_over_etat * (ray + cos_theta * normal);
        let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * normal;
        r_out_perp + r_out_parallel
    }
    pub fn to_radian(self) -> (f32, f32, f32) {
        let theta = self.0.e[1].acos();
        let phi = f32::atan2(-self.0.e[2], self.0.e[0]) + std::f32::consts::PI;
        (
            phi / std::f32::consts::TAU,
            theta / std::f32::consts::PI,
            self.length(),
        )
    }
}

// Point

impl std::ops::Sub<Point3> for Point3 {
    type Output = Dir3;
    fn sub(self, rhs: Point3) -> Self::Output {
        Dir3(self.0 - rhs.0)
    }
}

impl std::ops::Sub<Dir3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Dir3) -> Self::Output {
        Point3(self.0 - rhs.0)
    }
}

impl std::ops::Add<Dir3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Dir3) -> Self::Output {
        Point3(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<Dir3> for Point3 {
    fn add_assign(&mut self, rhs: Dir3) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign<Dir3> for Point3 {
    fn sub_assign(&mut self, rhs: Dir3) {
        *self = *self - rhs;
    }
}

// Dir
impl std::ops::Mul<Dir3> for f32 {
    type Output = Dir3;

    fn mul(self, rhs: Dir3) -> Self::Output {
        rhs * self
    }
}
