use crate::extensions::F32Extension;

pub trait GlamVec3Extension {
    fn almost_eq(&self, other: glam::Vec3) -> bool;
}

impl GlamVec3Extension for glam::Vec3 {
    fn almost_eq(&self, other: glam::Vec3) -> bool {
        self.x.almost_eq(other.x) && self.y.almost_eq(other.y) && self.z.almost_eq(other.z)
    }
}

pub trait GlamVec4Extension {
    fn almost_eq(&self, other: glam::Vec4) -> bool;
}

impl GlamVec4Extension for glam::Vec4 {
    fn almost_eq(&self, other: glam::Vec4) -> bool {
        self.x.almost_eq(other.x)
            && self.y.almost_eq(other.y)
            && self.z.almost_eq(other.z)
            && self.w.almost_eq(other.w)
    }
}
