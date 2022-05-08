use std::f64;
use std::ops::{Add, Index, Sub};

use crate::transforms::Mat3D;

#[derive(Debug, Clone, Copy)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Index<u8> for Vec3D {
    type Output = f64;

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of bound of Vec3D at index {}", u),
        }
    }
}

impl Add for Vec3D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D { x, y, z }
    }

    pub fn mul(self, rhs: Mat3D) -> Vec3D {
        Vec3D {
            x: self.x * rhs[0][0] + self.y * rhs[0][1] + self.z * rhs[0][2],
            y: self.x * rhs[1][0] + self.y * rhs[1][1] + self.z * rhs[1][2],
            z: self.x * rhs[2][0] + self.y * rhs[2][1] + self.z * rhs[2][2],
        }
    }
    pub fn normalized(self) -> Vec3D {
        let norm = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Vec3D::new(self.x / norm, self.y / norm, self.z / norm)
    }
}
