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
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 { x, y, z }
    }

    pub const ZERO: Point3 = Point3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
}

impl Dir3 {
    pub fn new(x: f32, y: f32, z: f32) -> Dir3 {
        Dir3 { x, y, z }
    }

    pub const ZERO: Dir3 = Dir3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_X: Dir3 = Dir3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UNIT_Y: Dir3 = Dir3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const UNIT_Z: Dir3 = Dir3 {
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

    pub fn dot(a: Dir3, b: Dir3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross(a: Dir3, b: Dir3) -> Dir3 {
        Dir3 {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn unit(self) -> Dir3 {
        self / self.length_squared()
    }
}

impl std::ops::Sub<Point3> for Point3 {
    type Output = Dir3;

    fn sub(self, rhs: Point3) -> Self::Output {
        Dir3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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
