use std::f64;
use std::ops::{Add, Index, Sub};

use crate::transforms::Mat3D;

#[derive(Debug, Clone, Copy)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
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
            w: self.w,
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
            w: self.w,
        }
    }
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3D {
        Vec3D { x, y, z, w: 1.0 }
    }

    pub fn mul(self, rhs: &Mat3D) -> Vec3D {
        Vec3D {
            x: self.x * rhs[0][0] + self.y * rhs[1][0] + self.z * rhs[2][0] + self.w * rhs[3][0],
            y: self.x * rhs[0][1] + self.y * rhs[1][1] + self.z * rhs[2][1] + self.w * rhs[3][1],
            z: self.x * rhs[0][2] + self.y * rhs[1][2] + self.z * rhs[2][2] + self.w * rhs[3][2],
            w: self.x * rhs[0][3] + self.y * rhs[1][3] + self.z * rhs[2][3] + self.w * rhs[3][3],
        }
    }

    pub fn scale(self, alpha: f64) -> Vec3D {
        Vec3D::new(self.x * alpha, self.y * alpha, self.z * alpha)
    }

    pub fn dot_product(self, other: Vec3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(self) -> f64 {
        self.dot_product(self).sqrt()
    }

    pub fn normalized(self) -> Vec3D {
        let l = self.length();
        Vec3D::new(self.x / l, self.y / l, self.z / l)
    }

    pub fn cross_product(self, other: Vec3D) -> Vec3D {
        Vec3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn intersect_plane(
        plane_pos: &Vec3D,
        plane_normal: &Vec3D,
        line_start: &Vec3D,
        line_end: &Vec3D,
    ) -> Vec3D {
        let plane_n = plane_normal.normalized();
        let plane_d = -plane_n.dot_product(*plane_pos);
        let ad = line_start.dot_product(plane_n);
        let bd = line_end.dot_product(plane_n);
        let t = (-plane_d - ad) / (bd - ad);

        let line_start_to_end = *line_end - *line_start;
        let line_to_intersect = line_start_to_end.scale(t);

        *line_start + line_to_intersect
    }
}
