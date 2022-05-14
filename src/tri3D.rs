use crate::{transforms::Mat3D, vec3D::Vec3D};
use std::{
    cmp::Ordering,
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Copy)]
pub struct Tri3D {
    pub a: Vec3D,
    pub b: Vec3D,
    pub c: Vec3D,
    pub l: f64,
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

impl IndexMut<u8> for Tri3D {
    fn index_mut(&mut self, u: u8) -> &mut Self::Output {
        match u {
            0 => &mut self.a,
            1 => &mut self.b,
            2 => &mut self.c,
            _ => panic!("Out of bound of Tri3D at index {}", u),
        }
    }
}

impl Ord for Tri3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let z1 = (self.a.z + self.b.z + self.c.z) / 3.0;
        let z2 = (other.a.z + other.b.z + other.c.z) / 3.0;
        if z1 < z2 {
            Ordering::Greater
        } else if z1 > z2 {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Tri3D {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Tri3D {}

impl PartialEq for Tri3D {
    fn eq(&self, other: &Self) -> bool {
        let z1 = (self.a.z + self.b.z + self.c.z) / 3.0;
        let z2 = (other.a.z + other.b.z + other.c.z) / 3.0;
        z1 == z2
    }
}

impl Tri3D {
    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Tri3D {
        Tri3D { a, b, c, l: 0.0 }
    }

    pub fn new_with_luminance(a: Vec3D, b: Vec3D, c: Vec3D, l: f64) -> Tri3D {
        Tri3D { a, b, c, l }
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
            l: 0.0,
        }
    }

    pub fn mul(&self, other: &Mat3D) -> Tri3D {
        Tri3D::new_with_luminance(
            self.a.mul(&other),
            self.b.mul(&other),
            self.c.mul(&other),
            self.l,
        )
    }

    pub fn clip(
        tri: &Tri3D,
        plane_pos: &Vec3D,
        plane_normal: &Vec3D,
    ) -> (u8, Option<Tri3D>, Option<Tri3D>) {
        let plane_n = plane_normal.normalized();

        let dist = |p: &Vec3D| -> f64 {
            plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z - plane_n.dot_product(*plane_pos)
        };

        let mut inside_points: [Option<&Vec3D>; 3] = [None, None, None];
        let mut inside_points_count = 0;
        let mut outside_points: [Option<&Vec3D>; 3] = [None, None, None];
        let mut outside_points_count = 0;

        let d0 = dist(&tri[0]);
        let d1 = dist(&tri[1]);
        let d2 = dist(&tri[2]);

        if d0 >= 0.0 {
            inside_points[inside_points_count] = Some(&tri[0]);
            inside_points_count += 1;
        } else {
            outside_points[outside_points_count] = Some(&tri[0]);
            outside_points_count += 1;
        }
        if d1 >= 0.0 {
            inside_points[inside_points_count] = Some(&tri[1]);
            inside_points_count += 1;
        } else {
            outside_points[outside_points_count] = Some(&tri[1]);
            outside_points_count += 1;
        }
        if d2 >= 0.0 {
            inside_points[inside_points_count] = Some(&tri[2]);
            inside_points_count += 1;
        } else {
            outside_points[outside_points_count] = Some(&tri[2]);
            outside_points_count += 1;
        }

        if inside_points_count == 0 {
            return (0, None, None);
        }
        if inside_points_count == 3 {
            return (1, Some(*tri), None);
        }

        if inside_points_count == 1 && outside_points_count == 2 {
            let out_tri = Tri3D::new_with_luminance(
                *inside_points[0].unwrap(),
                Vec3D::intersect_plane(
                    plane_pos,
                    &plane_n,
                    inside_points[0].unwrap(),
                    outside_points[0].unwrap(),
                ),
                Vec3D::intersect_plane(
                    plane_pos,
                    &plane_n,
                    inside_points[0].unwrap(),
                    outside_points[1].unwrap(),
                ),
                tri.l,
            );
            return (1, Some(out_tri), None);
        }

        // inside_points_count == 2 && outside_points_count == 1
        let out_tri1 = Tri3D::new_with_luminance(
            *inside_points[0].unwrap(),
            *inside_points[1].unwrap(),
            Vec3D::intersect_plane(
                plane_pos,
                &plane_n,
                inside_points[0].unwrap(),
                outside_points[0].unwrap(),
            ),
            tri.l,
        );
        let out_tri2 = Tri3D::new_with_luminance(
            *inside_points[1].unwrap(),
            out_tri1[2],
            Vec3D::intersect_plane(
                plane_pos,
                &plane_n,
                inside_points[1].unwrap(),
                outside_points[0].unwrap(),
            ),
            tri.l,
        );

        return (2, Some(out_tri1), Some(out_tri2));
    }
}
