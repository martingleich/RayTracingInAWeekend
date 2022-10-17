#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Dir3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3 { x, y, z }
    }
    pub fn new_from_arr(c: [f32; 3]) -> Self {
        Self::new(c[0], c[1], c[2])
    }
    pub const ORIGIN: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl Dir3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn new_from_arr(c: [f32; 3]) -> Self {
        Self::new(c[0], c[1], c[2])
    }

    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const RIGHT: Self = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const LEFT: Self = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UP: Self = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const DOWN: Self = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    pub const FORWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    pub const BACKWARD: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub fn length(self) -> f32 {
        f32::sqrt(self.length_squared())
    }

    pub fn length_squared(self) -> f32 {
        Self::dot(self, self)
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn unit(self) -> Self {
        self.with_length(1.0)
    }
    pub fn with_length(self, length: f32) -> Self {
        self * (length / self.length())
    }

    pub fn near_zero(self) -> bool {
        let eps : f32 = 1e-8;
        self.x.abs() < eps && self.y.abs() < eps && self.z.abs() < eps
    }
    pub fn near_zero_or_else(self, default: Dir3) -> Dir3 { if self.near_zero() {default} else {self} }
}

impl std::ops::Sub<Point3> for Point3 {
    type Output = Dir3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Dir3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Sub<Dir3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Dir3) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Add<Dir3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Dir3) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign<Dir3> for Point3 {
    fn add_assign(&mut self, rhs: Dir3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Mul<f32> for Point3 {
    type Output = Point3;

    fn mul(self, rhs: f32) -> Self::Output {
        Point3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Point3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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
        self * (1.0 / rhs)
    }
}

impl std::ops::Add<Dir3> for Dir3 {
    type Output = Dir3;

    fn add(self, rhs: Dir3) -> Self::Output {
        Dir3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<Dir3> for Dir3 {
    fn add_assign(&mut self, rhs: Dir3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Dir3> for Dir3 {
    type Output = Dir3;

    fn sub(self, rhs: Dir3) -> Self::Output {
        Dir3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign<Dir3> for Dir3 {
    fn sub_assign(&mut self, rhs: Dir3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<f32> for Dir3 {
    type Output = Dir3;

    fn mul(self, rhs: f32) -> Self::Output {
        Dir3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Dir3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
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
        self * (1.0 / rhs)
    }
}

impl std::ops::Neg for Dir3 {
    type Output = Dir3;

    fn neg(self) -> Self::Output {
        Dir3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
