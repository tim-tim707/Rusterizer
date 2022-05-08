use crate::vec2D::Vec2D;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy)]
pub struct Tri2D {
    pub a: Vec2D,
    pub b: Vec2D,
    pub c: Vec2D,
}

impl Index<u8> for Tri2D {
    type Output = Vec2D;

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.a,
            1 => &self.b,
            2 => &self.c,
            _ => panic!("Out of bound of Tri2D at index {}", u),
        }
    }
}

impl IndexMut<u8> for Tri2D {
    fn index_mut(&mut self, u: u8) -> &mut Self::Output {
        match u {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            _ => panic!("Out of bound of Tri2D at index {}", u),
        }
    }
}

impl Tri2D {
    pub fn new(a: Vec2D, b: Vec2D, c: Vec2D) -> Tri2D {
        Tri2D { a, b, c }
    }

    pub fn from_points(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Tri2D {
        Tri2D {
            a: Vec2D::new(a, b),
            b: Vec2D::new(c, d),
            c: Vec2D::new(e, f),
        }
    }
}
