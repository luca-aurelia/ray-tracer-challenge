use crate::extensions::F32Extension;
use std::ops::Sub;

use super::{vec2, Vec2, Vec3};

/// w == 1.0
#[derive(Debug, Clone, Copy)]
pub struct Point3 {
    pub vec: glam::Vec4,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Point3 {
        Point3 {
            vec: glam::Vec4::new(x, y, z, 1.0),
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
    pub fn xy(&self) -> Vec2 {
        vec2(self.x(), self.y())
    }
}

impl PartialEq for Point3 {
    fn eq(&self, other: &Self) -> bool {
        self.x().almost_eq(other.x())
            && self.y().almost_eq(other.y())
            && self.z().almost_eq(other.z())
            && self.w().almost_eq(other.w())
    }
}

impl Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, other: Point3) -> Vec3 {
        Vec3 {
            vec: self.vec - other.vec,
        }
    }
}

/// w == 1.0
pub fn pt3(x: f32, y: f32, z: f32) -> Point3 {
    Point3::new(x, y, z)
}

mod tests {
    #[test]
    fn points_have_a_w_value_of_1() {
        use super::*;
        let a = Point3::new(4.3, -4.2, 3.1);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
        assert_eq!(a.z(), 3.1);
        assert_eq!(a.w(), 1.0);
    }

    #[test]
    fn pt3_constructs_a_point() {
        use super::*;
        let point = pt3(4.0, -4.0, 3.0);
        assert_eq!(point.x(), 4.0);
        assert_eq!(point.y(), -4.0);
        assert_eq!(point.z(), 3.0);
        assert_eq!(point.w(), 1.0);
    }
}
