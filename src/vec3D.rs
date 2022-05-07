use std::f64;
use std::ops::{Add, Index, Sub};

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
