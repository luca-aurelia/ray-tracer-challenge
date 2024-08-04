use crate::extensions::F32Extension;
use std::ops::Sub;

use super::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Point2 {
    pub vec: glam::Vec2,
}

impl Point2 {
    pub fn new(x: f32, y: f32) -> Point2 {
        Point2 {
            vec: glam::Vec2::new(x, y),
        }
    }
    pub fn x(&self) -> f32 {
        self.vec.x
    }
    pub fn y(&self) -> f32 {
        self.vec.y
    }
    pub fn wh(&self) -> Point2 {
        let new_x = self.x().w();
        let new_y = self.y().h();
        pt2(new_x, new_y)
    }
    /// Flips points so that y == 0.0 is at the bottom of the canvas
    /// instead of the top.
    pub fn flip_y(&self) -> Point2 {
        pt2(self.x(), crate::CANVAS_HEIGHT - self.y())
    }
}

impl PartialEq for Point2 {
    fn eq(&self, other: &Self) -> bool {
        self.x().almost_eq(other.x()) && self.y().almost_eq(other.y())
    }
}

impl Sub<Point2> for Point2 {
    type Output = Vec2;

    fn sub(self, other: Point2) -> Vec2 {
        Vec2 {
            vec: self.vec - other.vec,
        }
    }
}

pub fn pt2(x: f32, y: f32) -> Point2 {
    Point2::new(x, y)
}

#[cfg(test)]
mod tests {
    #[test]
    fn points_have_a_w_value_of_1() {
        use super::*;
        let a = pt2(4.3, -4.2);
        assert_eq!(a.x(), 4.3);
        assert_eq!(a.y(), -4.2);
    }

    #[test]
    fn pt3_constructs_a_point() {
        use super::*;
        let point = pt2(4.0, -4.0);
        assert_eq!(point.x(), 4.0);
        assert_eq!(point.y(), -4.0);
    }
}
