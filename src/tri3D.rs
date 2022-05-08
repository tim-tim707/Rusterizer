use crate::{transforms::Mat3D, vec3D::Vec3D};
use std::ops::Index;

#[derive(Debug, Clone, Copy)]
pub struct Tri3D {
    pub a: Vec3D,
    pub b: Vec3D,
    pub c: Vec3D,
}

impl Index<u8> for Tri3D {
    type Output = Vec3D;

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            _ => panic!("Out of bound of Tri3D at index {}", u),
        }
    }
}

impl Tri3D {
    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Tri3D {
        Tri3D { a, b, c }
    }

    pub fn from_points(
        a: f64,
        b: f64,
        c: f64,
        d: f64,
        e: f64,
        f: f64,
        g: f64,
        h: f64,
        i: f64,
    ) -> Tri3D {
        Tri3D {
            a: Vec3D::new(a, b, c),
            b: Vec3D::new(d, e, f),
            c: Vec3D::new(g, h, i),
        }
    }

    pub fn mul(&self, other: Mat3D) -> Tri3D {
        Tri3D::new(self.a.mul(other), self.b.mul(other), self.c.mul(other))
    }
}
