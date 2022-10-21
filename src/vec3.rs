#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl<T: std::ops::Div<f32, Output = T> + Copy> std::ops::Div<f32> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point3 {
    v: Vec3<f32>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dir3 {
    v: Vec3<f32>,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            v: Vec3 { e: [x, y, z] },
        }
    }
    pub fn new_from_arr(c: [f32; 3]) -> Self {
        Self::new(c[0], c[1], c[2])
    }
    pub const ORIGIN: Self = Self {
        v: Vec3 { e: [0.0, 0.0, 0.0] },
    };
}

impl Dir3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            v: Vec3 { e: [x, y, z] },
        }
    }
    pub fn new_from_arr(c: [f32; 3]) -> Self {
        Self::new(c[0], c[1], c[2])
    }

    pub const ZERO: Self = Self {
        v: Vec3 { e: [0.0, 0.0, 0.0] },
    };
    pub const RIGHT: Self = Self {
        v: Vec3 { e: [1.0, 0.0, 0.0] },
    };
    pub const LEFT: Self = Self {
        v: Vec3 {
            e: [-1.0, 0.0, 0.0],
        },
    };
    pub const UP: Self = Self {
        v: Vec3 { e: [0.0, 1.0, 0.0] },
    };
    pub const DOWN: Self = Self {
        v: Vec3 {
            e: [0.0, -1.0, 0.0],
        },
    };
    pub const FORWARD: Self = Self {
        v: Vec3 {
            e: [0.0, 0.0, -1.0],
        },
    };
    pub const BACKWARD: Self = Self {
        v: Vec3 { e: [0.0, 0.0, 1.0] },
    };

    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f32 {
        Self::dot(self, self)
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.v.e[0] * b.v.e[0] + a.v.e[1] * b.v.e[1] + a.v.e[2] * b.v.e[2]
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            v: Vec3 {
                e: [
                    a.v.e[1] * b.v.e[2] - a.v.e[2] * b.v.e[1],
                    a.v.e[2] * b.v.e[0] - a.v.e[0] * b.v.e[2],
                    a.v.e[0] * b.v.e[1] - a.v.e[1] * b.v.e[0],
                ],
            },
        }
    }

    pub fn unit(self) -> Self {
        self.with_length(1.0)
    }
    pub fn with_length(self, length: f32) -> Self {
        self * (length / self.length())
    }

    pub fn near_zero(self) -> bool {
        let eps: f32 = 1e-8;
        self.v.e[0].abs() < eps && self.v.e[1].abs() < eps && self.v.e[2].abs() < eps
    }
    pub fn near_zero_or_else(self, default: Dir3) -> Dir3 {
        if self.near_zero() {
            default
        } else {
            self
        }
    }

    pub fn reflect(direction: Dir3, normal: Dir3) -> Dir3 {
        direction - (2.0 * Self::dot(direction, normal)) * normal
    }
}

// Point

impl std::ops::Sub<Point3> for Point3 {
    type Output = Dir3;
    fn sub(self, rhs: Point3) -> Self::Output {
        Self::Output { v: self.v - rhs.v }
    }
}

impl std::ops::Sub<Dir3> for Point3 {
    type Output = Point3;
    fn sub(self, rhs: Dir3) -> Self::Output {
        Self::Output { v: self.v - rhs.v }
    }
}

impl std::ops::Add<Dir3> for Point3 {
    type Output = Point3;
    fn add(self, rhs: Dir3) -> Self::Output {
        Self::Output { v: self.v + rhs.v }
    }
}

impl std::ops::AddAssign<Dir3> for Point3 {
    fn add_assign(&mut self, rhs: Dir3) {
        *self = *self + rhs;
    }
}

impl std::ops::Mul<f32> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { v: self.v * rhs }
    }
}

impl std::ops::MulAssign<f32> for Point3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<Point3> for f32 {
    type Output = Point3;

    fn mul(self, rhs: Point3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f32> for Point3 {
    type Output = Point3;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { v: self.v / rhs }
    }
}

// Dir

impl std::ops::Add<Dir3> for Dir3 {
    type Output = Dir3;

    fn add(self, rhs: Dir3) -> Self::Output {
        Self::Output { v: self.v + rhs.v }
    }
}

impl std::ops::AddAssign<Dir3> for Dir3 {
    fn add_assign(&mut self, rhs: Dir3) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub<Dir3> for Dir3 {
    type Output = Dir3;

    fn sub(self, rhs: Dir3) -> Self::Output {
        Self::Output { v: self.v - rhs.v }
    }
}

impl std::ops::SubAssign<Dir3> for Dir3 {
    fn sub_assign(&mut self, rhs: Dir3) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<f32> for Dir3 {
    type Output = Dir3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output { v: self.v * rhs }
    }
}

impl std::ops::MulAssign<f32> for Dir3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = *self * rhs;
    }
}

impl std::ops::Mul<Dir3> for f32 {
    type Output = Dir3;

    fn mul(self, rhs: Dir3) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Div<f32> for Dir3 {
    type Output = Dir3;
    fn div(self, rhs: f32) -> Self::Output {
        Self::Output { v: self.v / rhs }
    }
}

impl std::ops::Neg for Dir3 {
    type Output = Dir3;

    fn neg(self) -> Self::Output {
        Self::Output { v: -self.v }
    }
}
