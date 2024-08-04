#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod color;
use color::*;
mod canvas;
use ::glam::Vec3;
use canvas::Canvas;
mod extensions;
mod glam;
mod pixels;
mod ray_tracer_challenge;
mod sdf;
mod window;
use window::Window;

const WIDTH_U32: u32 = 100;
const HEIGHT_U32: u32 = 100;
const CANVAS_WIDTH: f32 = WIDTH_U32 as f32;
const CANVAS_HEIGHT: f32 = HEIGHT_U32 as f32;

struct Sketch {}

fn main() {
    env_logger::init();
    let window = Window::new(WIDTH_U32, HEIGHT_U32);
    let sketch = Sketch::new();
    window.run_event_loop(sketch);
}

impl Sketch {
    fn new() -> Self {
        Sketch {}
    }

    fn draw(&self, canvas: &mut Canvas) {
        ray_tracer_challenge::simulate_projectiles(canvas);
        // for xy in canvas.iter_points() {
        //     let color = oklcha(0.5, 1.0, 0.5, 1.0);
        //     canvas.set_pixel(xy, color);
        // }
    }
}

pub fn scene(xyz: Vec3) -> f32 {
    sdf::sphere(xyz)
}
