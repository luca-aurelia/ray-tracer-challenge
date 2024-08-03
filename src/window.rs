use error_iter::ErrorIter as _;
use log::error;
use pixels::Pixels;
use pixels::SurfaceTexture;
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::WindowBuilder;

use crate::canvas::Canvas;
use crate::{Sketch, HEIGHT, WIDTH};

pub struct Window {}

impl Window {
    pub fn new(sketch: Sketch) -> Self {
        let event_loop = EventLoop::new();
        let window = {
            let size = LogicalSize::new(WIDTH, HEIGHT);
            WindowBuilder::new()
                .with_title("Playground")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_resizable(false)
                .with_always_on_top(true)
                .build(&event_loop)
                .unwrap()
        };

        let mut canvas = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            let maybe_pixels = Pixels::new(WIDTH, HEIGHT, surface_texture);
            let pixels = match maybe_pixels {
                Ok(pixels) => pixels,
                Err(err) => {
                    log_error("Pixels::new", err);
                    panic!("Error creating Pixels");
                }
            };
            Canvas::new(pixels)
        };

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    // Close events
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                physical_key: KeyCode::Escape,
                                ..
                            },
                        ..
                    } => {
                        *control_flow = ControlFlow::Exit;
                    }

                    // Resize the window
                    WindowEvent::Resized(size) => {
                        if let Err(err) = canvas.pixels.resize_surface(size.width, size.height) {
                            log_error("pixels.resize_surface", err);
                            *control_flow = ControlFlow::Exit;
                        }
                    }

                    _ => {}
                },

                // Update internal state and request a redraw
                Event::MainEventsCleared => {
                    // world.update();
                    // window.request_redraw();
                }

                // Draw the current frame
                Event::RedrawRequested(_) => {
                    sketch.draw(&mut canvas);
                    if let Err(err) = canvas.pixels.render() {
                        log_error("pixels.render", err);
                        *control_flow = ControlFlow::Exit;
                    }
                }

                _ => {}
            }
        });
    }
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
