use std::ops::{Add, Sub};

use super::{Point2, Vec2};

impl Add<Point2> for Vec2 {
    type Output = Point2;

    fn add(self, other: Point2) -> Point2 {
        Point2 {
            vec: self.vec + other.vec,
        }
    }
}

impl Add<Vec2> for Point2 {
    type Output = Point2;

    fn add(self, other: Vec2) -> Point2 {
        Point2 {
            vec: self.vec + other.vec,
        }
    }
}

impl Sub<Vec2> for Point2 {
    type Output = Point2;

    fn sub(self, other: Vec2) -> Point2 {
        Point2 {
            vec: self.vec - other.vec,
        }
    }
}

mod tests {
    use super::super::{pt2, vec2};
    use super::*;

    #[test]
    fn adding_a_vector_and_a_point_yields_a_point() {
        let vector = vec2(3.0, -2.0);
        let point = pt2(-2.0, 3.0);
        let expected = pt2(1.0, 1.0);

        let actual = vector + point;
        assert_eq!(expected, actual);

        // Confirm that you can swap the order.
        let actual = point + vector;
        assert_eq!(expected, actual);
    }

    #[test]
    // This gives the vector from p2 to p1.
    fn subtracting_two_points_yields_a_vector() {
        let p1 = pt2(3.0, 2.0);
        let p2 = pt2(5.0, 6.0);
        let expected = vec2(-2.0, -4.0);
        let actual = p1 - p2;
        assert_eq!(expected, actual);
    }

    #[test]
    // This represents moving backwards along a vector.
    fn subtracting_a_vector_from_a_point_yields_a_point() {
        let p = pt2(3.0, 2.0);
        let v = vec2(5.0, 6.0);
        let expected = pt2(-2.0, -4.0);
        let actual = p - v;
        assert_eq!(expected, actual);
    }
}
