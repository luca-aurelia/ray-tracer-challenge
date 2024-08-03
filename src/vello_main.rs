use vello::kurbo::{Affine, Circle, Ellipse, Line, Rect, RoundedRect, Stroke};
// Copyright 2024 the Vello Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT
use anyhow::Result;
use std::num::NonZeroUsize;
use std::sync::Arc;
use vello::peniko::Color;
use vello::util::{RenderContext, RenderSurface};
use vello::wgpu;
use vello::{AaConfig, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::Window;

// Simple struct to hold the state of the renderer
pub struct ActiveRenderState<'s> {
    // The fields MUST be in this order, so that the surface is dropped before the window
    surface: RenderSurface<'s>,
    window: Arc<Window>,
}

enum RenderState<'s> {
    Active(ActiveRenderState<'s>),
    // Cache a window so that it can be reused when the app is resumed after being suspended
    Suspended(Option<Arc<Window>>),
}

struct SimpleVelloApp<'s> {
    x: u32,
    y: u32,

    // The vello RenderContext which is a global context that lasts for the
    // lifetime of the application
    context: RenderContext,

    // An array of renderers, one per wgpu device
    renderers: Vec<Option<Renderer>>,

    // State for our example where we store the winit Window and the wgpu Surface
    state: RenderState<'s>,

    // A vello Scene which is a data structure which allows one to build up a
    // description a scene to be drawn (with paths, fills, images, text, etc)
    // which is then passed to a renderer for rendering
    scene: Scene,
}

impl<'s> ApplicationHandler for SimpleVelloApp<'s> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached_window) = &mut self.state else {
            return;
        };

        // Get the winit window cached in a previous Suspended event or else create a new window
        let window = cached_window
            .take()
            .unwrap_or_else(|| create_winit_window(event_loop));

        // Create a vello Surface
        let size = window.inner_size();
        let surface_future = self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            wgpu::PresentMode::AutoVsync,
        );
        let surface = pollster::block_on(surface_future).expect("Error creating surface");

        // Create a vello Renderer for the surface (using its device id)
        self.renderers
            .resize_with(self.context.devices.len(), || None);
        self.renderers[surface.dev_id]
            .get_or_insert_with(|| create_vello_renderer(&self.context, &surface));

        // Save the Window and Surface to a state variable
        self.state = RenderState::Active(ActiveRenderState { window, surface });

        event_loop.set_control_flow(ControlFlow::Poll);
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        if let RenderState::Active(state) = &self.state {
            self.state = RenderState::Suspended(Some(state.window.clone()));
        }
        event_loop.set_control_flow(ControlFlow::Wait);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // Ignore the event (return from the function) if
        //   - we have no render_state
        //   - OR the window id of the event doesn't match the window id of our render_state
        //
        // Else extract a mutable reference to the render state from its containing option for use below
        let render_state = match &mut self.state {
            RenderState::Active(state) if state.window.id() == window_id => state,
            _ => return,
        };

        match event {
            // Exit the event loop when a close is requested (e.g. window's close button is pressed)
            WindowEvent::CloseRequested => event_loop.exit(),

            // Resize the surface when the window is resized
            WindowEvent::Resized(size) => {
                self.context
                    .resize_surface(&mut render_state.surface, size.width, size.height);
                render_state.window.request_redraw();
            }

            // This is where all the rendering happens
            WindowEvent::RedrawRequested => {
                // Empty the scene of objects to draw. You could create a new Scene each time, but in this case
                // the same Scene is reused so that the underlying memory allocation can also be reused.
                self.scene.reset();

                // Get the RenderSurface (surface + config)
                let surface = &render_state.surface;

                // Get the window size
                let width = surface.config.width;
                let height = surface.config.height;

                let mut last_draw_completed = std::time::Instant::now();
                let mut shapes_added_since_last_draw = 0;
                let mut total_shapes = 0;
                for x in 0..width {
                    for y in 0..height {
                        // Re-add the objects to draw to the scene.
                        add_shapes_to_scene(x, y, SquareColor::White, &mut self.scene);
                        shapes_added_since_last_draw += 1;
                        total_shapes += 1;
                        dbg!(total_shapes);
                        // dbg!("Adding shape.");

                        // Draw
                        let now = std::time::Instant::now();
                        let elapsed_since_last_draw = now - last_draw_completed;

                        let still_have_more_time_to_draw =
                            elapsed_since_last_draw < std::time::Duration::from_millis(10);
                        let reached_shape_limit = shapes_added_since_last_draw >= 1_000;
                        if still_have_more_time_to_draw && !reached_shape_limit {
                            continue;
                        }

                        shapes_added_since_last_draw = 0;

                        // dbg!("Drawing.");

                        // Get a handle to the device
                        let device_handle = &self.context.devices[surface.dev_id];

                        // Get the surface's texture
                        let surface_texture = surface
                            .surface
                            .get_current_texture()
                            .expect("failed to get surface texture");

                        // Render to the surface's texture
                        self.renderers[surface.dev_id]
                            .as_mut()
                            .unwrap()
                            .render_to_surface(
                                &device_handle.device,
                                &device_handle.queue,
                                &self.scene,
                                &surface_texture,
                                &vello::RenderParams {
                                    base_color: Color::TRANSPARENT, // Background color
                                    width,
                                    height,
                                    antialiasing_method: AaConfig::Msaa16,
                                },
                            )
                            .expect("failed to render to surface");

                        // Queue the texture to be presented on the surface
                        surface_texture.present();

                        device_handle.device.poll(wgpu::Maintain::Poll);

                        // self.scene.reset();
                        last_draw_completed = std::time::Instant::now();
                    }
                }

                // self.x += 1;
                // if self.x >= width {
                //     self.x = 0;
                //     self.y += 1;
                //     if self.y >= height {
                //         println!("Done.");
                //         return;
                //     }
                // }

                render_state.window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    // Setup a bunch of state:
    let mut app = SimpleVelloApp {
        x: 0,
        y: 0,
        context: RenderContext::new(),
        renderers: vec![],
        state: RenderState::Suspended(None),
        scene: Scene::new(),
    };

    // Create and run a winit event loop
    let event_loop = EventLoop::new()?;
    event_loop
        .run_app(&mut app)
        .expect("Couldn't run event loop");
    Ok(())
}

/// Helper function that creates a Winit window and returns it (wrapped in an Arc for sharing between threads)
fn create_winit_window(event_loop: &ActiveEventLoop) -> Arc<Window> {
    let attr = Window::default_attributes()
        .with_inner_size(LogicalSize::new(1044, 800))
        .with_resizable(true)
        .with_title("Vello Shapes");
    Arc::new(event_loop.create_window(attr).unwrap())
}

/// Helper function that creates a vello `Renderer` for a given `RenderContext` and `RenderSurface`
fn create_vello_renderer(render_cx: &RenderContext, surface: &RenderSurface) -> Renderer {
    Renderer::new(
        &render_cx.devices[surface.dev_id].device,
        RendererOptions {
            surface_format: Some(surface.format),
            use_cpu: false,
            antialiasing_support: vello::AaSupport::all(),
            num_init_threads: NonZeroUsize::new(1),
        },
    )
    .expect("Couldn't create renderer")
}

enum SquareColor {
    Black,
    White,
}

/// Add shapes to a vello scene. This does not actually render the shapes, but adds them
/// to the Scene data structure which represents a set of objects to draw.
fn add_shapes_to_scene(x: u32, y: u32, color: SquareColor, scene: &mut Scene) {
    // Draw a filled ellipse
    let x0 = x as f64;
    let y0 = y as f64;
    let x1 = x0 + 1.0;
    let y1 = y0 + 1.0;
    let rect = Rect::new(x0, y0, x1, y1);
    let color = match color {
        SquareColor::Black => Color::rgb(0.0, 0.0, 0.0),
        SquareColor::White => Color::rgb(1.0, 1.0, 1.0),
    };
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        color,
        None,
        &rect,
    )

    // let width_f64 = width as f64;
    // let height_f64 = height as f64;

    // let x0 = width_f64 * 0.25;
    // let y0 = height_f64 * 0.25;
    // let x1 = width_f64 * 0.75;
    // let y1 = height_f64 * 0.75;

    // let rect = Rect::new(x0, y0, x1, y1);
    // let color = Color::rgb(0.7961, 0.651, 0.9686);
    // scene.fill(
    //     vello::peniko::Fill::NonZero,
    //     Affine::IDENTITY,
    //     color,
    //     None,
    //     &rect,
    // );
}