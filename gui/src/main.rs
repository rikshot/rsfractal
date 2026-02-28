use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use renderer::MandelbrotRenderer;
use rsfractal_mandelbrot::mandelbrot::{Coloring, Mandelbrot, Rendering, rect_from_position};
use rsfractal_mandelbrot::range::Range;
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
    cursor_position: (f64, f64),
    gpu_rendering: bool,
    last_frame: Option<Instant>,
    fps: f64,
}

const MIN_WIDTH: u32 = 1280;
const MIN_HEIGHT: u32 = 720;

impl App<'_> {
    fn update_title(&self) {
        if let Some(window) = &self.window {
            let renderer = if self.gpu_rendering { "GPU" } else { "CPU" };
            let fps = self.fps;
            let coloring = match self.mandelbrot.coloring {
                Coloring::Palette => {
                    let name = &self.mandelbrot.palettes()[self.mandelbrot.selected_palette].0;
                    format!("Palette | (P)alette: {name}")
                }
                Coloring::LCH => "LCH".to_string(),
            };
            let iterations = self.mandelbrot.max_iterations;
            if self.gpu_rendering {
                window.set_title(&format!("rsfractal | (M)ode: {renderer} | (C)oloring: {coloring} | Iterations(↑↓): {iterations} | {fps:.1} fps"));
            } else {
                let rendering = &self.mandelbrot.rendering;
                window.set_title(&format!("rsfractal | (M)ode: {renderer} | (R)endering: {rendering} | (C)oloring: {coloring} | Iterations(↑↓): {iterations} | {fps:.1} fps"));
            }
        }
    }
}

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
                self.mandelbrot.width as u32,
                self.mandelbrot.height as u32,
                surface_texture,
            ) {
                self.window = Some(window);
                self.renderer = Some(MandelbrotRenderer::new(&pixels, &self.mandelbrot));
                self.pixels = Some(pixels);
                self.update_title();
                self.window.as_ref().unwrap().request_redraw();
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
                        if !self.gpu_rendering {
                            self.mandelbrot.set_resolution(size.width as usize, size.height as usize);
                            let _ = pixels.resize_buffer(size.width, size.height);
                        }
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key: PhysicalKey::Code(key), state: ElementState::Pressed, .. },
                ..
            } => {
                match key {
                    KeyCode::Escape => event_loop.exit(),
                    KeyCode::KeyC => {
                        self.mandelbrot.coloring = match self.mandelbrot.coloring {
                            Coloring::Palette => Coloring::LCH,
                            Coloring::LCH => Coloring::Palette,
                        };
                        if let (Some(pixels), Some(renderer)) = (&self.pixels, &mut self.renderer) {
                            renderer.update_coloring(pixels.device(), pixels.queue(), &self.mandelbrot);
                        }
                        self.update_title();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                    KeyCode::KeyM => {
                        self.gpu_rendering = !self.gpu_rendering;
                        if !self.gpu_rendering {
                            if let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels) {
                                let size = window.inner_size();
                                self.mandelbrot.set_resolution(size.width as usize, size.height as usize);
                                let _ = pixels.resize_buffer(size.width, size.height);
                            }
                        }
                        self.update_title();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                    KeyCode::KeyR => {
                        self.mandelbrot.rendering = match self.mandelbrot.rendering {
                            Rendering::Smooth => Rendering::Fast,
                            Rendering::Fast => Rendering::Smooth,
                        };
                        self.update_title();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                    KeyCode::KeyP => {
                        self.mandelbrot.selected_palette = (self.mandelbrot.selected_palette + 1) % self.mandelbrot.palettes().len();
                        if let (Some(pixels), Some(renderer)) = (&self.pixels, &mut self.renderer) {
                            renderer.update_coloring(pixels.device(), pixels.queue(), &self.mandelbrot);
                        }
                        self.update_title();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                    KeyCode::ArrowUp => {
                        self.mandelbrot.max_iterations = (self.mandelbrot.max_iterations * 2).min(100000);
                        self.update_title();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                    KeyCode::ArrowDown => {
                        self.mandelbrot.max_iterations = (self.mandelbrot.max_iterations / 2).max(10);
                        self.update_title();
                        if let Some(window) = &self.window {
                            window.request_redraw();
                        }
                    }
                    _ => ()
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = (position.x, position.y);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::PixelDelta(physical_position) = delta {
                    let zoom_factor = if physical_position.y > 0.0 {
                        0.9
                    } else if physical_position.y < 0.0 {
                        1.1
                    } else {
                        return;
                    };
                    if let Some(window) = &self.window {
                        let size = window.inner_size();
                        let (cx, cy) = self.cursor_position;
                        let rect = rect_from_position(&self.mandelbrot.position, &self.mandelbrot.zoom);
                        let width_range = Range::new(0.0, size.width as f32);
                        let height_range = Range::new(0.0, size.height as f32);
                        let real_range = Range::new(rect.start.x, rect.end.x);
                        let imaginary_range = Range::new(rect.start.y, rect.end.y);
                        let target_re = Range::scale(&width_range, cx as f32, &real_range);
                        let target_im = Range::scale(&height_range, cy as f32, &imaginary_range);
                        self.mandelbrot.zoom.x *= zoom_factor;
                        self.mandelbrot.zoom.y *= zoom_factor;
                        self.mandelbrot.position.x = target_re + (self.mandelbrot.position.x - target_re) * zoom_factor;
                        self.mandelbrot.position.y = target_im + (self.mandelbrot.position.y - target_im) * zoom_factor;
                        window.request_redraw();
                    }
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
                let now = Instant::now();
                if let Some(last) = self.last_frame {
                    let dt = now.duration_since(last).as_secs_f64();
                    if dt > 0.0 {
                        self.fps = 1.0 / dt;
                    }
                }
                self.last_frame = Some(now);

                if let Some(pixels) = &mut self.pixels {
                    if self.gpu_rendering {
                        if let (Some(renderer), Some(window)) = (&self.renderer, &self.window) {
                            let size = window.inner_size();
                            pixels
                                .render_with(|encoder, render_target, context| {
                                    renderer.set_params(&context.queue, &self.mandelbrot, size.width as f32, size.height as f32);
                                    renderer.render(encoder, render_target, (0, 0, size.width, size.height));
                                    Ok(())
                                })
                                .unwrap();
                        }
                    } else {
                        self.mandelbrot.render(pixels.frame_mut());
                        pixels.render().unwrap();
                    }
                }
                self.update_title();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let mut app = App { gpu_rendering: true, ..Default::default() };
    event_loop.run_app(&mut app)?;
    Ok(())
}
