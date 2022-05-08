use std::f64;
use std::ops::{Add, Index, Sub};

use crate::transforms::Mat2D;
#[derive(Debug, Clone, Copy)]
pub struct Vec2D {
    pub x: f64,
    pub y: f64,
}

impl Index<u8> for Vec2D {
    type Output = f64;

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Out of bound of Vec2D at index {}", u),
        }
    }
}

impl Add for Vec2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vec2D {
    pub fn new(x: f64, y: f64) -> Vec2D {
        Vec2D { x, y }
    }

    pub fn mul(self, rhs: Mat2D) -> Vec2D {
        Vec2D {
            x: self.x * rhs[0][0] + self.y * rhs[0][1],
            y: self.x * rhs[1][0] + self.y * rhs[1][1],
        }
    }

    pub fn normalized(self) -> Vec2D {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        Vec2D::new(self.x / norm, self.y / norm)
    }
}
