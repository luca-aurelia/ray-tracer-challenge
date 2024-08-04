use crate::extensions::F32Extension;
use std::ops::{Add, Mul, Sub};

type PaletteSrgb = palette::Srgb<f32>;

#[derive(Debug, Clone, Copy)]
pub struct Rgb {
    palette_srgb: PaletteSrgb,
}

impl Rgb {
    pub fn black() -> Rgb {
        Rgb {
            palette_srgb: PaletteSrgb::new(0.0, 0.0, 0.0),
        }
    }
    pub fn new(red: f32, green: f32, blue: f32) -> Rgb {
        Rgb {
            palette_srgb: PaletteSrgb::new(red, green, blue),
        }
    }
    pub fn red(&self) -> f32 {
        self.palette_srgb.red
    }
    pub fn green(&self) -> f32 {
        self.palette_srgb.green
    }
    pub fn blue(&self) -> f32 {
        self.palette_srgb.blue
    }
    pub fn components(&self) -> [f32; 3] {
        [self.red(), self.green(), self.blue()]
    }
}

impl Add<Rgb> for Rgb {
    type Output = Rgb;

    fn add(self, other: Rgb) -> Rgb {
        Rgb {
            palette_srgb: self.palette_srgb + other.palette_srgb,
        }
    }
}

impl Sub<Rgb> for Rgb {
    type Output = Rgb;

    fn sub(self, other: Rgb) -> Rgb {
        Rgb {
            palette_srgb: self.palette_srgb - other.palette_srgb,
        }
    }
}

impl Mul<f32> for Rgb {
    type Output = Rgb;

    fn mul(self, scalar: f32) -> Rgb {
        Rgb {
            palette_srgb: self.palette_srgb * scalar,
        }
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Rgb;

    fn mul(self, other: Rgb) -> Rgb {
        Rgb {
            palette_srgb: self.palette_srgb * other.palette_srgb,
        }
    }
}

impl PartialEq for Rgb {
    fn eq(&self, other: &Rgb) -> bool {
        self.palette_srgb.red.almost_eq(other.palette_srgb.red)
            && self.palette_srgb.green.almost_eq(other.palette_srgb.green)
            && self.palette_srgb.blue.almost_eq(other.palette_srgb.blue)
    }
}

impl From<&[u8]> for Rgb {
    fn from(slice: &[u8]) -> Rgb {
        Rgb::new(
            slice[0] as f32 / 255.0,
            slice[1] as f32 / 255.0,
            slice[2] as f32 / 255.0,
        )
    }
}

impl Into<PaletteSrgb> for Rgb {
    fn into(self) -> PaletteSrgb {
        self.palette_srgb
    }
}

impl Into<palette::Srgba<f32>> for Rgb {
    fn into(self) -> palette::Srgba<f32> {
        palette::Srgba::<f32>::new(self.red(), self.green(), self.blue(), 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extensions::F32Extension;

    #[test]
    fn test_rgb() {
        let rgb = Rgb::new(-0.5, 0.4, 1.7);
        assert_eq!(rgb.red(), -0.5);
        assert_eq!(rgb.green(), 0.4);
        assert_eq!(rgb.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let a = Rgb::new(0.9, 0.6, 0.75);
        let b = Rgb::new(0.7, 0.1, 0.25);
        let actual = a + b;
        assert!(actual.red().almost_eq(1.6));
        assert!(actual.green().almost_eq(0.7));
        assert!(actual.blue().almost_eq(1.0));
    }

    #[test]
    fn subtracting_colors() {
        let a = Rgb::new(0.9, 0.6, 0.75);
        let b = Rgb::new(0.7, 0.1, 0.25);
        let actual = a - b;
        assert!(actual.red().almost_eq(0.2));
        assert!(actual.green().almost_eq(0.5));
        assert!(actual.blue().almost_eq(0.5));
    }

    #[test]
    fn multiplying_color_by_scalar() {
        let a = Rgb::new(0.2, 0.3, 0.4);
        let actual = a * 2.0;
        assert!(actual.red().almost_eq(0.4));
        assert!(actual.green().almost_eq(0.6));
        assert!(actual.blue().almost_eq(0.8));
    }

    #[test]
    fn multiplying_color_by_color() {
        let c1 = Rgb::new(1.0, 0.2, 0.4);
        let c2 = Rgb::new(0.9, 1.0, 0.1);
        let actual = c1 * c2;
        let expected = Rgb::new(0.9, 0.2, 0.04);
        assert_eq!(actual, expected);
    }
}
