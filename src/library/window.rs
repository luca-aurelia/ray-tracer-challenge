use crate::crate_wrappers::pixels::Pixels;
use crate::library::canvas::Canvas;
use crate::Sketch;
use log::error;
use tao::dpi::LogicalSize;
use tao::event::{Event, KeyEvent, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::keyboard::KeyCode;
use tao::window::WindowBuilder;

pub struct Window {
    event_loop: EventLoop<()>,
    pub canvas: Canvas,

    // We need to hold on to this because if we drop it,
    // Tao doesn't actually render the window.
    #[allow(dead_code)]
    tao_window: tao::window::Window,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let tao_window = {
            let size = LogicalSize::new(width, height);
            WindowBuilder::new()
                .with_title("Playground")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .with_resizable(false)
                .with_always_on_top(true)
                .build(&event_loop)
                .unwrap()
        };

        let canvas = {
            #[cfg(not(test))]
            let pixels = {
                use crate::crate_wrappers::pixels::SurfaceTexture;

                let window_size = tao_window.inner_size();
                let surface_texture =
                    SurfaceTexture::new(window_size.width, window_size.height, &tao_window);
                let maybe_pixels = Pixels::new(width, height, surface_texture);
                match maybe_pixels {
                    Ok(pixels) => pixels,
                    Err(err) => {
                        log_error("Pixels::new", err);
                        panic!("Error creating Pixels");
                    }
                }
            };

            #[cfg(test)]
            let pixels = Pixels::new(width, height);

            Canvas::new(width, height, pixels)
        };

        Window {
            event_loop,
            tao_window,
            canvas,
        }
    }

    pub fn run_event_loop(mut self, sketch: Sketch) {
        self.event_loop.run(move |event, _, control_flow| {
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
                        if let Err(err) = self.canvas.pixels.resize_surface(size.width, size.height)
                        {
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
                    sketch.draw(&mut self.canvas);
                    if let Err(err) = self.canvas.pixels.render() {
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
    use error_iter::ErrorIter as _;

    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}
