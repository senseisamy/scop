use std::ops::{Add, Index, IndexMut, Mul};

#[derive(Debug, Clone)]
pub struct Vec4(pub [f32; 4]);

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl Mul for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self([
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3],
        ])
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self([self[0] * rhs, self[1] * rhs, self[2] * rhs, self[3] * rhs])
    }
}
