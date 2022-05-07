use crate::vec3D::Vec3D;
use std::ops::Index;

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
