use glam::Vec4Swizzles;

use crate::extensions::F32Extension;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// w == 0.0
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub vec: glam::Vec4,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            vec: glam::Vec4::new(x, y, z, 0.0),
        }
    }
    pub fn x(&self) -> f32 {
        self.vec.x
    }
    pub fn y(&self) -> f32 {
        self.vec.y
    }
    pub fn z(&self) -> f32 {
        self.vec.z
    }
    pub fn w(&self) -> f32 {
        self.vec.w
    }
    pub fn length(&self) -> f32 {
        self.vec.length()
    }
    pub fn normalize(&self) -> Vec3 {
        Vec3 {
            vec: self.vec.normalize(),
        }
    }
    /// When comparing unit vectors:
    ///
    /// 1.0 means the vectors are the same
    /// 0.0 means the vectors are orthogonal
    /// -1.0 means the vectors are diametrically opposed
    ///
    /// This is the cosine of the angle between them.
    pub fn dot(&self, other: Self) -> f32 {
        self.vec.dot(other.vec)
    }
    /// Returns a new vector that is perpendicular to both
    /// of the input vectors.
    ///
    /// If you swap the order of the input vectors, it negates
    /// the output.
    pub fn cross(&self, other: Self) -> Vec3 {
        let self_xyz = self.vec.xyz();
        let other_xyz = other.vec.xyz();
        let cross = self_xyz.cross(other_xyz);

        Vec3::new(cross.x, cross.y, cross.z)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x().almost_eq(other.x())
            && self.y().almost_eq(other.y())
            && self.z().almost_eq(other.z())
            && self.w().almost_eq(other.w())
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            vec: self.vec + other.vec,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            vec: self.vec - other.vec,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { vec: -self.vec }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            vec: self.vec * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            vec: self.vec / scalar,
        }
    }
}

/// w == 0.0
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert!(a == b);
    }

    #[test]
    fn vectors_have_a_w_value_value_of_0() {
        let a = Vec3::new(4.3, -4.2, 3.1);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 0.0);
    }

    #[test]
    fn vec3_constructs_a_vector() {
        let vector = vec3(4.0, -4.0, 3.0);
        assert_eq!(vector.x(), 4.0);
        assert_eq!(vector.y(), -4.0);
        assert_eq!(vector.z(), 3.0);
        assert_eq!(vector.w(), 0.0);
    }

    #[test]
    // This represents moving backwards along a vector.
    fn subtracting_two_vectors() {
        let v1 = vec3(3.0, 2.0, 1.0);
        let v2 = vec3(5.0, 6.0, 7.0);
        let actual = v1 - v2;
        let expected = vec3(-2.0, -4.0, -6.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn negating_a_vector() {
        let v = vec3(1.0, -2.0, 3.0);
        let actual = -v;
        let expected = vec3(-1.0, 2.0, -3.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let a = vec3(1.0, -2.0, 3.0);
        let actual = a * 3.5;
        let expected = vec3(3.5, -7.0, 10.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let a = vec3(1.0, -2.0, 3.0);
        let actual = a * 0.5;
        let expected = vec3(0.5, -1.0, 1.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let a = vec3(1.0, -2.0, 3.0);
        let actual = a / 2.0;
        let expected = vec3(0.5, -1.0, 1.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn magnitude_of_a_vector() {
        let v = vec3(1.0, 0.0, 0.0);
        assert_eq!(v.length(), 1.0);

        let v = vec3(0.0, 1.0, 0.0);
        assert_eq!(v.length(), 1.0);

        let v = vec3(0.0, 0.0, 1.0);
        assert_eq!(v.length(), 1.0);

        let v = vec3(1.0, 2.0, 3.0);
        assert_eq!(v.length(), 14.0_f32.sqrt());

        let v = vec3(-1.0, -2.0, -3.0);
        assert_eq!(v.length(), 14.0_f32.sqrt());
    }

    #[test]
    fn normalizing_vectors_returns_the_correct_value() {
        let v = vec3(4.0, 0.0, 0.0);
        let actual = v.normalize();
        let expected = vec3(1.0, 0.0, 0.0);
        assert_eq!(actual, expected);

        let v = vec3(1.0, 2.0, 3.0);
        let actual = v.normalize();
        let expected = vec3(0.26726, 0.53452, 0.80178);
        assert_eq!(actual, expected);
    }

    #[test]
    fn normalizing_vectors_returns_a_vector_with_magnitude_1() {
        let v = vec3(1.0, 2.0, 3.0);
        let actual = v.normalize().length();
        let expected = 1.0;
        assert!(actual.almost_eq(expected));
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = vec3(1.0, 2.0, 3.0);
        let b = vec3(2.0, 3.0, 4.0);
        let actual = a.dot(b);
        let expected = 20.0;
        assert!(actual.almost_eq(expected));
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let v1 = vec3(1.0, 2.0, 3.0);
        let v2 = vec3(2.0, 3.0, 4.0);

        let actual = v1.cross(v2);
        let expected = vec3(-1.0, 2.0, -1.0);
        assert_eq!(actual, expected);

        let actual = v2.cross(v1);
        let expected = vec3(1.0, -2.0, 1.0);
        assert_eq!(actual, expected);
    }
}
