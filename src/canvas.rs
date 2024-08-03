use palette::{FromColor, Oklcha, Srgba};
use pixels::Pixels;
use std::time::{Duration, Instant};

use crate::glam::{vec2, Vec2};
use crate::{HEIGHT, WIDTH};

const WIDTH_F32: f32 = WIDTH as f32;
const HEIGHT_F32: f32 = HEIGHT as f32;

pub struct Canvas {
    pub pixels: Pixels,
    last_render_at: Instant,
    max_time_since_last_render: Duration,
}

pub fn width() -> f32 {
    WIDTH_F32
}

pub fn height() -> f32 {
    HEIGHT_F32
}

impl Canvas {
    pub fn new(pixels: Pixels) -> Self {
        let target_fps = 60.0;
        let max_time_since_last_render = Duration::from_secs_f64(1.0 / target_fps);
        Self {
            pixels,
            last_render_at: Instant::now(),
            max_time_since_last_render,
        }
    }

    pub fn set_pixel(&mut self, xy: Vec2, oklcha: Oklcha) {
        let srgba: Srgba<u8> = Srgba::from_color(oklcha).into_format();
        let srgba_components: [u8; 4] = palette::cast::into_array(srgba);
        self.update_frame(xy, srgba_components);
        self.render_if_needed();
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Vec2> {
        (0..WIDTH).flat_map(move |x| (0..HEIGHT).map(move |y| vec2(x as f32, y as f32)))
    }

    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`.
    /// This maps to Srgba<u8> in the palette crate.
    fn update_frame(&mut self, xy: Vec2, srgba: [u8; 4]) {
        let x_u32 = xy.x.round() as u32;
        let y_u32 = xy.y.round() as u32;

        let index = (x_u32 + y_u32 * WIDTH) as usize * 4; // Times four because each pixel has four channels.
        let frame = self.pixels.frame_mut();
        frame[index..index + 4].copy_from_slice(&srgba);
    }

    fn render_if_needed(&mut self) {
        let now = Instant::now();
        let time_since_last_render = now - self.last_render_at;

        if time_since_last_render > self.max_time_since_last_render {
            self.pixels.render().expect("pixels.render() failed.");
            self.last_render_at = Instant::now();
        }
    }
}
