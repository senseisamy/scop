use super::Camera;
use crate::math::{Mat4, Vec3};

impl Camera {
    pub fn view_matrix(&self) -> Mat4 {
        let w = self.direction.normalize();
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
}
