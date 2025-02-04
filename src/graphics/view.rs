use super::Camera;
use crate::math::{Mat4, Vec3};

impl Camera {
    pub fn direction_view_matrix(&self, direction: Vec3) -> Mat4 {
        let w = direction.normalize();
        let u = Vec3::cross(
            &w,
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        )
        .normalize();
        let v = Vec3::cross(&w, &u);

        let mut view_matrix = Mat4::identity();
        view_matrix[0][0] = u.x;
        view_matrix[1][0] = u.y;
        view_matrix[2][0] = u.z;
        view_matrix[0][1] = v.x;
        view_matrix[1][1] = v.y;
        view_matrix[2][1] = v.z;
        view_matrix[0][2] = w.x;
        view_matrix[1][2] = w.y;
        view_matrix[2][2] = w.z;
        view_matrix[3][0] = -Vec3::dot(&u, &self.position);
        view_matrix[3][1] = -Vec3::dot(&v, &self.position);
        view_matrix[3][2] = -Vec3::dot(&w, &self.position);
        view_matrix
    }

    pub fn target_dir(&self) -> Vec3 {
        (self.position - self.target).normalize()
    }

    pub fn update_position(&mut self) {
        self.position = Vec3 {
            x: self.target.x + self.distance * self.phi.cos() * self.theta.cos(),
            y: self.target.y + self.distance * self.phi.sin(),
            z: self.target.z + self.distance * self.phi.cos() * self.theta.sin(),
        }
    }
}
