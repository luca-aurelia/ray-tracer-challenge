use crate::extensions::F32Extension;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// w == 0.0
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub vec: glam::Vec2,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 {
            vec: glam::Vec2::new(x, y),
        }
    }
    pub fn x(&self) -> f32 {
        self.vec.x
    }
    pub fn y(&self) -> f32 {
        self.vec.y
    }
    pub fn length(&self) -> f32 {
        self.vec.length()
    }
    pub fn normalize(&self) -> Vec2 {
        Vec2 {
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
    pub fn wh(&self) -> Vec2 {
        let new_x = self.x() * crate::CANVAS_WIDTH;
        let new_y = self.y() * crate::CANVAS_HEIGHT;
        vec2(new_x, new_y)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x().almost_eq(other.x()) && self.y().almost_eq(other.y())
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            vec: self.vec + other.vec,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            vec: self.vec - other.vec,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 { vec: -self.vec }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            vec: self.vec * scalar,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, scalar: f32) -> Vec2 {
        Vec2 {
            vec: self.vec / scalar,
        }
    }
}

/// w == 0.0
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_eq() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(1.0, 2.0);
        assert!(a == b);
    }

    #[test]
    fn vectors_have_a_w_value_value_of_0() {
        let a = Vec2::new(4.3, -4.2);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
    }

    #[test]
    fn vec2_constructs_a_vector() {
        let vector = vec2(4.0, -4.0);
        assert_eq!(vector.x(), 4.0);
        assert_eq!(vector.y(), -4.0);
    }

    #[test]
    // This represents moving backwards along a vector.
    fn subtracting_two_vectors() {
        let v1 = vec2(3.0, 2.0);
        let v2 = vec2(5.0, 6.0);
        let actual = v1 - v2;
        let expected = vec2(-2.0, -4.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn negating_a_vector() {
        let v = vec2(1.0, -2.0);
        let actual = -v;
        let expected = vec2(-1.0, 2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let a = vec2(1.0, -2.0);
        let actual = a * 3.5;
        let expected = vec2(3.5, -7.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiplying_a_vector_by_a_fraction() {
        let a = vec2(1.0, -2.0);
        let actual = a * 0.5;
        let expected = vec2(0.5, -1.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn dividing_a_vector_by_a_scalar() {
        let a = vec2(1.0, -2.0);
        let actual = a / 2.0;
        let expected = vec2(0.5, -1.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn magnitude_of_a_vector() {
        let v = vec2(1.0, 0.0);
        assert_eq!(v.length(), 1.0);

        let v = vec2(0.0, 1.0);
        assert_eq!(v.length(), 1.0);

        let v = vec2(0.0, 0.0);
        assert_eq!(v.length(), 0.0);

        let v = vec2(1.0, 2.0);
        assert_eq!(v.length(), 5.0_f32.sqrt());

        let v = vec2(-1.0, -2.0);
        assert_eq!(v.length(), 5.0_f32.sqrt());
    }

    #[test]
    fn normalizing_vectors_returns_the_correct_value() {
        let v = vec2(4.0, 0.0);
        let actual = v.normalize();
        let expected = vec2(1.0, 0.0);
        assert_eq!(actual, expected);

        let v = vec2(1.0, 2.0);
        let actual = v.normalize();
        let expected = vec2(0.4472136, 0.8944272);
        assert_eq!(actual, expected);
    }

    #[test]
    fn normalizing_vectors_returns_a_vector_with_magnitude_1() {
        let v = vec2(1.0, 2.0);
        let actual = v.normalize().length();
        let expected = 1.0;
        assert!(actual.almost_eq(expected));
    }

    #[test]
    fn dot_product_of_two_vectors() {
        let a = vec2(1.0, 2.0);
        let b = vec2(2.0, 3.0);
        let actual = a.dot(b);
        let expected = 8.0;
        assert!(actual.almost_eq(expected));
    }
}
