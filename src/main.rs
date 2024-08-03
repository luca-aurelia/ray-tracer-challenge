#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod color;
use color::*;
mod canvas;
use canvas::Canvas;
mod glam;
use crate::glam::Vec2;
mod window;
use window::Window;

const WIDTH: u32 = 1_000;
const HEIGHT: u32 = 1_000;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct Sketch {}

fn main() {
    env_logger::init();
    let sketch = Sketch::new();
    Window::new(sketch);
}

impl Sketch {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        Sketch {}
    }

    /// Draw the `World` state to the frame buffer.
    fn draw(&self, canvas: &mut Canvas) {
        for xy in canvas.iter_points() {
            let color = oklcha(0.5, 1.0, 0.5, 1.0);
            canvas.set_pixel(xy, color);
        }
    }
}
