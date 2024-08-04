use std::ops::{Add, Sub};

use super::{Point3, Vec3};

impl Add<Point3> for Vec3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3 {
            vec: self.vec + other.vec,
        }
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vec3) -> Point3 {
        Point3 {
            vec: self.vec + other.vec,
        }
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vec3) -> Point3 {
        Point3 {
            vec: self.vec - other.vec,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{pt3, vec3};
    use super::*;

    #[test]
    fn adding_a_vector_and_a_point_yields_a_point() {
        let vector = vec3(3.0, -2.0, 5.0);
        let point = pt3(-2.0, 3.0, 1.0);
        let expected = pt3(1.0, 1.0, 6.0);

        let actual = vector + point;
        assert_eq!(expected, actual);

        // Confirm that you can swap the order.
        let actual = point + vector;
        assert_eq!(expected, actual);
    }

    #[test]
    // This gives the vector from p2 to p1.
    fn subtracting_two_points_yields_a_vector() {
        let p1 = pt3(3.0, 2.0, 1.0);
        let p2 = pt3(5.0, 6.0, 7.0);
        let expected = vec3(-2.0, -4.0, -6.0);
        let actual = p1 - p2;
        assert_eq!(expected, actual);
    }

    #[test]
    // This represents moving backwards along a vector.
    fn subtracting_a_vector_from_a_point_yields_a_point() {
        let p = pt3(3.0, 2.0, 1.0);
        let v = vec3(5.0, 6.0, 7.0);
        let expected = pt3(-2.0, -4.0, -6.0);
        let actual = p - v;
        assert_eq!(expected, actual);
    }
}
