use core::default::Default;
use core::iter::Sum;
use core::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn norm(&self) -> f32 {
        libm::sqrtf(dot_product(self, self))
    }

    pub fn normalized(&self) -> Vec3 {
        self.mul(self.norm().recip())
    }

    pub fn dist(&self, rhs: &Vec3) -> f32 {
        let diff = *self - *rhs;
        diff.norm()
    }

    pub fn dist_observer(&self, rhs: &Vec3) -> f32 {
        let diff = *self - *rhs;
        dot_product(&diff, &diff)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Sum<Vec3> for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Default::default(), |acc, val| acc + val)
    }
}

pub fn dot_product(v1: &Vec3, v2: &Vec3) -> f32 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn cross_product(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_vec3_add() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(-4.0, 15.0, 7.0), vec1 + vec2);
    }

    #[test]
    fn test_vec3_mul() {
        let vec = Vec3::new(1.0, 5.0, 7.0);
        assert_eq!(Vec3::new(3.0, 15.0, 21.0), vec * 3.0);
    }

    #[test]
    fn test_vec3_norm() {
        let vec = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(5.0, vec.norm());
    }

    #[test]
    fn test_vec3_normalized() {
        let vec = Vec3::new(3.0, 4.0, 0.0);
        assert_eq!(1.0, vec.normalized().norm());
    }

    #[test]
    fn test_vec3_scalar() {
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(1.0, 1.0, 0.0);
        assert_eq!(1.0, dot_product(&vec1, &vec2));
    }
}
