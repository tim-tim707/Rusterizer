use crate::transforms::Mat3D;
use crate::tri3D::Tri3D;
use crate::vec3D::Vec3D;

pub struct Camera {
    pub yaw: f64,
    pub pos: Vec3D,
    pub look_dir: Vec3D,
    view_mat: Mat3D,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pos: Vec3D::new(0.0, 0.0, 0.0),
            look_dir: Vec3D::new(1.0, 0.0, 0.0),
            yaw: 0.0,
            view_mat: Mat3D::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn compute_view(&mut self) {
        // yaw -> 0,0,1 * rotY -> lookDir -> target -> (matCamera)-1 -> view_mat
        let camera_rotation_y = Mat3D::rot_y(self.yaw);
        self.look_dir = Vec3D::new(0.0, 0.0, 1.0).mul(&camera_rotation_y);

        let target = self.pos + self.look_dir;
        let camera_mat = Mat3D::point_at(self.pos, target, Vec3D::new(0.0, 1.0, 0.0));
        self.view_mat = camera_mat.quick_inverse();
    }

    pub fn to_view(&self, other: Tri3D) -> Tri3D {
        other.mul(&self.view_mat)
    }
}
