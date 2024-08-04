use crate::library::canvas::Canvas;
// use crate::prelude::*;
use crate::ray_tracer_challenge;

pub const CANVAS_WIDTH: f32 = 100.0;
pub const CANVAS_HEIGHT: f32 = 100.0;

pub struct Sketch {}

impl Sketch {
    pub fn new() -> Self {
        Sketch {}
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        ray_tracer_challenge::simulate_projectiles(canvas);
        // for xy in canvas.iter_points() {
        //     let color = oklcha(0.5, 1.0, 0.5, 1.0);
        //     canvas.set_pixel(xy, color);
        // }
    }
}

// pub fn scene(xyz: Vec3) -> f32 {
//     sdf::sphere(xyz)
// }
