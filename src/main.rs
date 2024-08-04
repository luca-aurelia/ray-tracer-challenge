#![deny(clippy::all)]
#![forbid(unsafe_code)]

use library::window::Window;
use prelude::*;
use sketch::Sketch;

mod crate_wrappers;
mod extensions;
mod library;
mod prelude;
mod ray_tracer_challenge;
mod sketch;

fn main() {
    env_logger::init();
    let window = Window::new(CANVAS_WIDTH.round() as u32, CANVAS_HEIGHT.round() as u32);
    let sketch = Sketch::new();
    window.run_event_loop(sketch);
}
