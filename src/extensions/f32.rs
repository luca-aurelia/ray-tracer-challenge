pub trait F32Extension {
    fn almost_eq(&self, other: f32) -> bool;
    fn denormalize(&self, min: f32, max: f32) -> f32;
    fn w(&self) -> f32;
    fn h(&self) -> f32;
}

impl F32Extension for f32 {
    fn almost_eq(&self, other: f32) -> bool {
        let epsilon = 0.00001;
        let difference = self - other;
        difference.abs() < epsilon
    }
    fn denormalize(&self, min: f32, max: f32) -> f32 {
        self * (max - min) + min
    }
    /// Denormalize to canvas width.
    fn w(&self) -> f32 {
        self.denormalize(0.0, crate::CANVAS_WIDTH)
    }
    /// Denormalize to canvas height.
    fn h(&self) -> f32 {
        self.denormalize(0.0, crate::CANVAS_HEIGHT)
    }
}

mod tests {
    #[test]
    fn f32_almost_eq() {
        use super::*;
        let a = 1.0;
        let b = 1.0;
        assert!(a.almost_eq(b));
    }
}
