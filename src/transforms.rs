use std::{f64::consts::PI, ops::Index};

use crate::vec3D::Vec3D;

pub struct Mat2D {
    data: [[f64; 3]; 3],
}

impl Index<u8> for Mat2D {
    type Output = [f64; 3];

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.data[0],
            1 => &self.data[1],
            2 => &self.data[2],
            _ => panic!("Out of bound of Mat2D at index {}", u),
        }
    }
}

impl Mat2D {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Mat2D {
        Mat2D {
            data: [[a, b, 0.0], [c, d, 0.0], [0.0, 0.0, 1.0]],
        }
    }

    pub fn rot_x(alpha: f64) -> Mat2D {
        Mat2D {
            data: [
                [alpha.cos(), -alpha.sin(), 0.0],
                [alpha.sin(), alpha.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

pub struct Mat3D {
    data: [[f64; 4]; 4],
}

impl Index<u8> for Mat3D {
    type Output = [f64; 4];

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.data[0],
            1 => &self.data[1],
            2 => &self.data[2],
            3 => &self.data[3],
            _ => panic!("Out of bound of Mat3D at index {}", u),
        }
    }
}

impl Mat3D {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> Mat3D {
        Mat3D {
            data: [
                [a, b, c, 0.0],
                [d, e, f, 0.0],
                [g, h, i, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rot_x(alpha: f64) -> Mat3D {
        Mat3D {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, alpha.cos(), alpha.sin(), 0.0],
                [0.0, -alpha.sin(), alpha.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn rot_y(beta: f64) -> Mat3D {
        Mat3D {
            data: [
                [beta.cos(), 0.0, beta.sin(), 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-beta.sin(), 0.0, beta.cos(), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn rot_z(gamma: f64) -> Mat3D {
        Mat3D {
            data: [
                [gamma.cos(), gamma.sin(), 0.0, 0.0],
                [-gamma.sin(), gamma.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Mat3D {
        Mat3D {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [x, y, z, 1.0],
            ],
        }
    }

    pub fn projection(fov: f64, aspect_ratio: f64, near: f64, far: f64) -> Mat3D {
        let fov_rad = 1.0 / (fov * PI / 360.0).tan();
        Mat3D {
            data: [
                [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
                [0.0, fov_rad, 0.0, 0.0],
                [0.0, 0.0, far / (far - near), 1.0],
                [0.0, 0.0, (-far * near) / (far - near), 0.0],
            ],
        }
    }

    pub fn quick_inverse(&self) -> Mat3D {
        Mat3D {
            data: [
                [self[0][0], self[1][0], self[2][0], 0.0],
                [self[0][1], self[1][1], self[2][1], 0.0],
                [self[0][2], self[1][2], self[2][2], 0.0],
                [
                    -(self[3][0] * self[0][0] + self[3][1] * self[0][1] + self[3][2] * self[0][2]),
                    -(self[3][0] * self[1][0] + self[3][1] * self[1][1] + self[3][2] * self[1][2]),
                    -(self[3][0] * self[2][0] + self[3][1] * self[2][1] + self[3][2] * self[2][2]),
                    1.0,
                ],
            ],
        }
    }

    pub fn point_at(pos: &Vec3D, target: &Vec3D, up: &Vec3D) -> Mat3D {
        let new_forward = (*target - *pos).normalized();

        let tmp = new_forward.scale(up.dot_product(new_forward));
        let new_up = (*up - tmp).normalized();

        let new_right = new_up.cross_product(new_forward);

        Mat3D {
            data: [
                [new_right.x, new_right.y, new_right.z, 0.0],
                [new_up.x, new_up.y, new_up.z, 0.0],
                [new_forward.x, new_forward.y, new_forward.z, 0.0],
                [pos.x, pos.y, pos.z, 1.0],
            ],
        }
    }
}
