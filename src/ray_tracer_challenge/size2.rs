pub struct Size2 {
    glam_vec2: glam::Vec2,
}

impl Size2 {
    pub fn new(width: f32, height: f32) -> Size2 {
        Size2 {
            glam_vec2: glam::Vec2::new(width, height),
        }
    }
    pub fn width(&self) -> f32 {
        self.glam_vec2.x
    }
    pub fn height(&self) -> f32 {
        self.glam_vec2.y
    }
}

pub fn size2(width: f32, height: f32) -> Size2 {
    Size2::new(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_and_height() {
        let size2 = Size2::new(1.0, 2.0);
        assert_eq!(size2.width(), 1.0);
        assert_eq!(size2.height(), 2.0);
    }
}
