use std::sync::Arc;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use rsfractal_mandelbrot::mandelbrot::{Config, render};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{ElementState, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

#[derive(Default)]
struct App<'a> {
    config: Config,
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'a>>,
    mouse_position: Option<PhysicalPosition<f64>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        if let Ok(window) = event_loop.create_window(
            Window::default_attributes()
                .with_title("rsfractal")
                .with_inner_size(size)
                .with_min_inner_size(size),
        ) {
            let window = Arc::new(window);
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            if let Ok(pixels) = Pixels::new(WIDTH, HEIGHT, surface_texture) {
                window.request_redraw();
                self.window = Some(window);
                self.pixels = Some(pixels);
            } else {
                event_loop.exit();
            }
        } else {
            event_loop.exit();
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
                if let Some(window) = &self.window {
                    if let Some(pixels) = &mut self.pixels {
                        if pixels.resize_surface(size.width, size.height).is_err() {
                            event_loop.exit();
                        } else {
                            window.request_redraw();
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let KeyEvent { physical_key, .. } = event;
                if let PhysicalKey::Code(KeyCode::Escape) = physical_key {
                    event_loop.exit()
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Some(position);
            }
            WindowEvent::MouseInput {
                button,
                state: ElementState::Released,
                ..
            } => {
                if let Some(window) = &self.window {
                    if let Some(pixels) = &self.pixels {
                        if let Some(mouse_position) = self.mouse_position {
                            let (x, y) = pixels.window_pos_to_pixel(mouse_position.into()).unwrap();
                            let zoom_factor = match button {
                                MouseButton::Left => 0.25,
                                MouseButton::Right => 1.75,
                                _ => 1.0,
                            };
                            self.config.zoom(x as f64, y as f64, zoom_factor);
                            window.request_redraw()
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    render(&self.config, pixels.frame_mut());
                    pixels.render().unwrap();
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
