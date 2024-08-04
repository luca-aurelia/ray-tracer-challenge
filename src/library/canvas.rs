use camino::Utf8Path;
use image::ImageResult;
use palette::{FromColor, Oklcha, Srgba};
use std::time::{Duration, Instant};

use crate::crate_wrappers::pixels::{Pixels, NUM_COMPONENTS_IN_COLOR};
use crate::ray_tracer_challenge::{pt2, size2, Point2, Rgb, Size2};

/// `pixels` stores colors internally as four instances of `u8`.
pub struct Canvas {
    pub pixels: Pixels,
    width: u32,
    height: u32,
    last_render_at: Instant,
    max_time_since_last_render: Duration,
}

impl Canvas {
    pub fn new(width: u32, height: u32, pixels: Pixels) -> Self {
        let target_fps = 60.0;
        let max_time_since_last_render = Duration::from_secs_f64(1.0 / target_fps);
        Self {
            width,
            height,
            pixels,
            last_render_at: Instant::now(),
            max_time_since_last_render,
        }
    }

    pub fn set_pixel(&mut self, xy: Point2, oklcha: Oklcha) {
        let srgba = Srgba::from_color(oklcha);
        self.update_frame_slice(xy, srgba);
    }

    pub fn set_pixel_rgb(&mut self, xy: Point2, rgb: Rgb) {
        self.update_frame_slice(xy, rgb.into());
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point2> {
        let width = self.width;
        let height = self.height;
        (0..width).flat_map(move |x| (0..height).map(move |y| pt2(x as f32, y as f32)))
    }

    pub fn rgb_at(&self, xy: Point2) -> Option<Rgb> {
        let components = self.frame_slice(xy)?;
        let rgb = components.into();
        Some(rgb)
    }

    pub fn iter_pixels(&self) -> impl Iterator<Item = Pixel> + '_ {
        self.pixels
            .frame()
            .chunks_exact(NUM_COMPONENTS_IN_COLOR)
            .enumerate()
            .map(|(index, chunk)| Pixel {
                xy: self.xy_from_frame_index(index),
                rgb: chunk.into(),
            })
    }

    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`.
    /// This maps to Srgba<u8> in the palette crate.
    fn update_frame_slice(&mut self, xy: Point2, srgba: Srgba<f32>) {
        let srgba_u8: Srgba<u8> = srgba.into_format();
        let srgba_components: [u8; NUM_COMPONENTS_IN_COLOR] = palette::cast::into_array(srgba_u8);
        if let Some(frame_slice) = self.frame_slice_mut(xy) {
            frame_slice.copy_from_slice(&srgba_components);
            self.render_if_needed();
        }
    }

    fn frame_slice(&self, xy: Point2) -> Option<&[u8]> {
        let start = self.frame_index(xy);
        let end = start + NUM_COMPONENTS_IN_COLOR;
        let frame = self.pixels.frame();

        if end > frame.len() {
            return None;
        }

        let slice = &frame[start..end];
        Some(slice)
    }

    fn frame_slice_mut(&mut self, xy: Point2) -> Option<&mut [u8]> {
        let start = self.frame_index(xy);
        let end = start + NUM_COMPONENTS_IN_COLOR;
        let frame = self.pixels.frame_mut();

        if end > frame.len() {
            return None;
        }

        let slice = &mut frame[start..end];
        Some(slice)
    }

    fn render_if_needed(&mut self) {
        let now = Instant::now();
        let time_since_last_render = now - self.last_render_at;

        if time_since_last_render > self.max_time_since_last_render {
            self.pixels.render().expect("pixels.render() failed.");
            self.last_render_at = Instant::now();
        }
    }

    /// Given an xy coordinate, return the index of the first component of the color at that pixel.
    fn frame_index(&self, xy: Point2) -> usize {
        let x_usize = xy.x().round() as usize;
        let y_usize = xy.y().round() as usize;
        let width_usize = self.width as usize;

        // Incorporate NUM_COMPONENTS_IN_COLOR to account for the fact that each pixel has multiple components.
        (y_usize * width_usize + x_usize) * NUM_COMPONENTS_IN_COLOR
    }

    fn xy_from_frame_index(&self, index: usize) -> Point2 {
        let width_usize = self.width as usize;
        let x = index % width_usize;
        let y = index / width_usize;
        pt2(x as f32, y as f32)
    }

    pub fn save_image(&self, path: &Utf8Path) -> ImageResult<()> {
        let mut image_buffer = image::ImageBuffer::new(self.width, self.height);
        for pixel in self.iter_pixels() {
            let x = pixel.xy.x().round() as u32;
            let y = pixel.xy.y().round() as u32;
            let image_buffer_pixel = image_buffer.get_pixel_mut(x, y);

            let srgba_f32: Srgba<f32> = pixel.rgb.into();
            let srgba_u8: Srgba<u8> = srgba_f32.into_format();
            let r = srgba_u8.red;
            let g = srgba_u8.green;
            let b = srgba_u8.blue;

            *image_buffer_pixel = image::Rgba([r, g, b, 255]);
        }
        image_buffer.save(path)
    }

    pub fn width(&self) -> f32 {
        self.width as f32
    }

    pub fn height(&self) -> f32 {
        self.height as f32
    }

    pub fn size(&self) -> Size2 {
        size2(self.width as f32, self.height as f32)
    }
}

pub struct Pixel {
    xy: Point2,
    rgb: Rgb,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray_tracer_challenge::pt2;

    #[test]
    fn colors_default_to_black() {
        let width = 10;
        let height = 20;
        let pixels = Pixels::new(width, height);
        let canvas = Canvas::new(width, height, pixels);
        let black = Rgb::black();
        for pixel in canvas.iter_pixels() {
            assert_eq!(pixel.rgb, black);
        }
        assert_eq!(canvas.pixels.width, 10);
        assert_eq!(canvas.pixels.height, 20);
    }

    #[test]
    fn set_pixel_rgb() {
        let width = 10;
        let height = 20;
        let pixels = Pixels::new(width, height);
        let mut canvas = Canvas::new(width, height, pixels);

        let red = Rgb::new(1.0, 0.0, 0.0);
        let xy = pt2(2.0, 3.0);
        canvas.set_pixel_rgb(xy, red);

        let rgb = canvas.rgb_at(xy).expect("Pixel not found.");
        assert_eq!(rgb, red);
    }

    #[test]
    fn save_image() {
        let width = 10;
        let height = 20;
        let pixels = Pixels::new(width, height);
        let mut canvas = Canvas::new(width, height, pixels);

        let red = Rgb::new(1.0, 0.0, 0.0);
        let xy = pt2(2.0, 3.0);
        canvas.set_pixel_rgb(xy, red);

        let path = Utf8Path::new("src/tests/save_image_actual_output.webp");
        canvas.save_image(path).expect("Failed to save image.");

        let actual_bytes = std::fs::read(path).expect("Failed to read file.");
        let expected_bytes = include_bytes!("../tests/save_image_expected_output.webp");
        assert_eq!(actual_bytes, expected_bytes);
        // std::fs::remove_file(path).expect("Failed to remove file.");
    }
}
