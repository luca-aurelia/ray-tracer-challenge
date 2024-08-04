use crate::glam::Vec3;

pub fn sphere(xyz: Vec3) -> f32 {
    xyz.length() - 1.0
}
