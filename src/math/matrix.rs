use std::ops::{Add, Index, IndexMut, Mul, MulAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Index<usize> for Mat4 {
    type Output = [f32];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            [
                self[1][0] + rhs[0][0],
                self[0][1] + rhs[0][1],
                self[0][2] + rhs[0][2],
                self[0][3] + rhs[0][3],
            ],
            [
                self[1][0] + rhs[1][0],
                self[1][1] + rhs[1][1],
                self[1][2] + rhs[1][2],
                self[1][3] + rhs[1][3],
            ],
            [
                self[2][0] + rhs[2][0],
                self[2][1] + rhs[2][1],
                self[2][2] + rhs[2][2],
                self[2][3] + rhs[2][3],
            ],
            [
                self[3][0] + rhs[3][0],
                self[3][1] + rhs[3][1],
                self[3][2] + rhs[3][2],
                self[3][3] + rhs[3][3],
            ],
        ])
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut m = Mat4::default();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    m[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }

        m
    }
}

impl MulAssign for Mat4 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

#[allow(dead_code)]
impl Mat4 {
    pub fn identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut scale_matrix = Self::identity();
        scale_matrix[0][0] = x;
        scale_matrix[1][1] = y;
        scale_matrix[2][2] = z;

        scale_matrix
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut translation_matrix = Self::identity();
        translation_matrix[0][3] = x;
        translation_matrix[1][3] = y;
        translation_matrix[2][3] = z;

        translation_matrix
    }

    pub fn rotate_x(angle: f32) -> Self {
        let mut rotation_matrix = Self::identity();
        rotation_matrix[1][1] = angle.cos();
        rotation_matrix[1][2] = -angle.sin();
        rotation_matrix[2][1] = angle.sin();
        rotation_matrix[2][2] = angle.cos();

        rotation_matrix
    }

    pub fn rotate_y(angle: f32) -> Self {
        let mut rotation_matrix = Self::identity();
        rotation_matrix[0][0] = angle.cos();
        rotation_matrix[0][2] = angle.sin();
        rotation_matrix[2][0] = -angle.sin();
        rotation_matrix[2][2] = angle.cos();

        rotation_matrix
    }

    pub fn rotation_z(angle: f32) -> Self {
        let mut rotation_matrix = Self::identity();
        rotation_matrix[0][0] = angle.cos();
        rotation_matrix[0][1] = -angle.sin();
        rotation_matrix[1][0] = angle.sin();
        rotation_matrix[1][1] = angle.cos();

        rotation_matrix
    }

    pub fn perspective(vertical_fov: f32, aspect_ratio: f32, z_near: f32, z_far: f32) -> Self {
        let inv_length = 1.0 / (z_near - z_far);
        let f = 1.0 / (0.5 * vertical_fov).tan();
        let a = f / aspect_ratio;
        let b = (z_near + z_far) * inv_length;
        let c = (2.0 * z_near * z_far) * inv_length;

        Self ([
            [a, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, b, -1.0],
            [0.0, 0.0, c, 0.0]
        ])
    }
}
