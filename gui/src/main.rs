use std::sync::Arc;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use renderer::MandelbrotRenderer;
use rsfractal_mandelbrot::mandelbrot::{Mandelbrot, rect_from_position};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

mod renderer;

#[derive(Default)]
struct App<'a> {
    mandelbrot: Mandelbrot,
    renderer: Option<MandelbrotRenderer>,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    mouse_button: bool,
}

const MIN_WIDTH: u32 = 1280;
const MIN_HEIGHT: u32 = 720;

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize::new(MIN_WIDTH as f64, MIN_HEIGHT as f64);
        if let Ok(window) = event_loop.create_window(
            Window::default_attributes()
                .with_title("rsfractal")
                .with_min_inner_size(size),
        ) {
            let window = Arc::new(window);
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            if let Ok(pixels) = Pixels::new(
                self.mandelbrot.width() as u32,
                self.mandelbrot.height() as u32,
                surface_texture,
            ) {
                window.request_redraw();
                self.window = Some(window);
                self.renderer = Some(MandelbrotRenderer::new(&pixels));
                self.pixels = Some(pixels);
            } else {
                event_loop.exit();
            }
        } else {
            event_loop.exit();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let Some(window) = &self.window {
            if let DeviceEvent::MouseMotion { delta } = event {
                if self.mouse_button {
                    let rect = rect_from_position(&self.mandelbrot.position, &self.mandelbrot.zoom);
                    self.mandelbrot.position.x -= delta.0 as f32 * rect.width() / 1000.0;
                    self.mandelbrot.position.y -= delta.1 as f32 * rect.height() / 1000.0;
                    window.request_redraw();
                }
            } else if let DeviceEvent::MouseWheel { delta } = event {
                if let MouseScrollDelta::PixelDelta(physical_position) = delta {
                    if physical_position.y > 0.0 {
                        self.mandelbrot.zoom.x *= 0.9;
                        self.mandelbrot.zoom.y *= 0.9;
                    } else if physical_position.y < 0.0 {
                        self.mandelbrot.zoom.x *= 1.1;
                        self.mandelbrot.zoom.y *= 1.1;
                    }
                }
                window.request_redraw();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(window) = &self.window
                    && let Some(pixels) = &mut self.pixels
                {
                    if pixels.resize_surface(size.width, size.height).is_err() {
                        event_loop.exit();
                    } else {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let KeyEvent { physical_key, .. } = event;
                if let PhysicalKey::Code(KeyCode::Escape) = physical_key {
                    event_loop.exit()
                }
            }
            WindowEvent::MouseInput {
                button: MouseButton::Left,
                state,
                ..
            } => {
                self.mouse_button = state == ElementState::Pressed;
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels
                    && let Some(renderer) = &self.renderer
                {
                    pixels
                        .render_with(|encoder, render_target, context| {
                            renderer.set_ranges(&context.queue, &self.mandelbrot.ranges());
                            renderer.render(encoder, render_target, context.scaling_renderer.clip_rect());
                            Ok(())
                        })
                        .unwrap();
                }
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut app = App::default();
    event_loop.run_app(&mut app)?;
    Ok(())
}
