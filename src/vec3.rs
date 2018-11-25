use std::fmt::{self, Display, Formatter};
use std::ops::*;

use math::float::Float;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    x: Float,
    y: Float,
    z: Float,
}

impl Vec3 {
    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> Float {
        self.x
    }

    pub fn y(&self) -> Float {
        self.y
    }

    pub fn z(&self) -> Float {
        self.z
    }

    pub fn r(&self) -> Float {
        self.x
    }

    pub fn g(&self) -> Float {
        self.y
    }

    pub fn b(&self) -> Float {
        self.z
    }
}

impl Vec3 {
    pub fn length(&self) -> Float {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> Float {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn make_unit_vector(&mut self) {
        *self /= self.length()
    }

    pub fn dot(&self, other: &Vec3) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Index<usize> for Vec3 {
    type Output = Float;
    fn index(&self, idx: usize) -> &Float {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("invalid index {}", idx),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Float {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("invalid index {}", idx),
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Mul<Float> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: Float) -> Vec3 {
        Vec3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Div<Float> for Vec3 {
    type Output = Vec3;
    fn div(self, t: Float) -> Vec3 {
        Vec3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl Mul<Vec3> for Float {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, t: Float) {
        self.x *= t;
        self.y *= t;
        self.z *= t;
    }
}

impl DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, t: Float) {
        self.x /= t;
        self.y /= t;
        self.z /= t;
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
