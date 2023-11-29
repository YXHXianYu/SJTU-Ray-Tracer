pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }
    pub fn from(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn abs(&self) -> f64 {
        self.abs2().sqrt()
    }
    pub fn abs2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

use std::ops::Add;
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

use std::ops::Sub;
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self: Self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

use std::ops::Mul;
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self: Self, other: Self) -> Self {
        Self {
            x: self.x * other.x, 
            y: self.y * other.y, 
            z: self.z * other.z,
        }
    }
}
impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar, 
            y: self.y * scalar, 
            z: self.z * scalar,
        }
    }
}

use std::ops::Div;
impl Div for Vec3 {
    type Output = Self;

    fn div(self: Self, other: Self) -> Self {
        Self {
            x: self.x / other.x, 
            y: self.y / other.y, 
            z: self.z / other.z,
        }
    }
}
impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar, 
            y: self.y / scalar, 
            z: self.z / scalar,
        }
    }
}

use std::ops::Neg;
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x, 
            y: -self.y, 
            z: -self.z,
        }
    }
}

use std::ops::Index;
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x, 
            1 => &self.y, 
            2 => &self.z, 
            _ => panic!("Index out of bounds"),
        }
    }
}

use std::ops::AddAssign;
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

use std::ops::SubAssign;
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

use std::ops::MulAssign;
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

use std::ops::DivAssign;
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}