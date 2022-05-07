use crate::vec2D::Vec2D;
use std::ops::Index;

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

impl Tri2D {
    pub fn new(a: Vec2D, b: Vec2D, c: Vec2D) -> Tri2D {
        Tri2D { a, b, c }
    }
}
