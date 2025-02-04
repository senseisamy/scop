use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use super::Mat4;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("vector index out of bound"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("vector index out of bound"),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Mat4> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Mat4) -> Self::Output {
        Self {
            x: self.x * rhs[0][0] + self.y * rhs[0][1] + self.z * rhs[0][2],
            y: self.x * rhs[1][0] + self.y * rhs[1][1] + self.z * rhs[1][2],
            z: self.x * rhs[2][0] + self.y * rhs[2][1] + self.z * rhs[2][2],
        }
    }
}

impl MulAssign<Mat4> for Vec3 {
    fn mul_assign(&mut self, rhs: Mat4) {
        *self = *self * rhs;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Vec3 {
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        return Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        };
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn to_array(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn from(array: &[f32; 3]) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}
