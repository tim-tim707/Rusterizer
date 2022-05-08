use std::ops::Index;

pub struct Mat2D {
    data: [[f64; 2]; 2],
}

impl Index<u8> for Mat2D {
    type Output = [f64; 2];

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.data[0],
            1 => &self.data[1],
            _ => panic!("Out of bound of Mat2D at index {}", u),
        }
    }
}

impl Mat2D {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Mat2D {
        Mat2D {
            data: [[a, b], [c, d]],
        }
    }

    pub fn rot_x(alpha: f64) -> Mat2D {
        Mat2D {
            data: [[alpha.cos(), -alpha.sin()], [alpha.sin(), alpha.cos()]],
        }
    }
}

pub struct Mat3D {
    data: [[f64; 3]; 3],
}

impl Index<u8> for Mat3D {
    type Output = [f64; 3];

    fn index(&self, u: u8) -> &Self::Output {
        match u {
            0 => &self.data[0],
            1 => &self.data[1],
            2 => &self.data[2],
            _ => panic!("Out of bound of Mat3D at index {}", u),
        }
    }
}

impl Mat3D {
    pub fn new(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64, g: f64, h: f64, i: f64) -> Mat3D {
        Mat3D {
            data: [[a, b, c], [d, e, f], [g, h, i]],
        }
    }

    pub fn rot_x(alpha: f64) -> Mat3D {
        Mat3D {
            data: [
                [alpha.cos(), -alpha.sin(), 0.0],
                [alpha.sin(), alpha.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
    pub fn rot_y(beta: f64) -> Mat3D {
        Mat3D {
            data: [
                [beta.cos(), 0.0, beta.sin()],
                [0.0, 1.0, 0.0],
                [-beta.sin(), 0.0, beta.cos()],
            ],
        }
    }
    pub fn rot_z(gamma: f64) -> Mat3D {
        Mat3D {
            data: [
                [1.0, 0.0, 0.0],
                [0.0, gamma.cos(), -gamma.sin()],
                [0.0, gamma.sin(), gamma.cos()],
            ],
        }
    }
}
