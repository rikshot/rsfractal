use std::sync::Arc;
use std::time::Instant;

use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use renderer::MandelbrotRenderer;
use rsfractal_mandelbrot::mandelbrot::{Coloring, Mandelbrot, Rendering};
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
    fn debug_dump(&mut self) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let dir = format!("debug_dump_{timestamp}");
        std::fs::create_dir_all(&dir).unwrap();

        let json = serde_json::to_string_pretty(&self.mandelbrot).unwrap();
        std::fs::write(format!("{dir}/state.json"), &json).unwrap();

        // Use window size for both renders so they're directly comparable
        let (width, height) = self.window.as_ref().map_or(
            (self.mandelbrot.width as u32, self.mandelbrot.height as u32),
            |w| { let s = w.inner_size(); (s.width, s.height) },
        );

        // CPU render
        let saved_size = (self.mandelbrot.width, self.mandelbrot.height);
        self.mandelbrot.set_resolution(width as usize, height as usize);
        let mut cpu_buf = vec![0u8; (width * height * 4) as usize];
        self.mandelbrot.render(&mut cpu_buf);
        self.mandelbrot.set_resolution(saved_size.0, saved_size.1);

        let img = image::RgbaImage::from_raw(width, height, cpu_buf).unwrap();
        img.save(format!("{dir}/cpu_render.png")).unwrap();

        // GPU render
        let use_perturbation = self.mandelbrot.perturbation_active;

        if let (Some(pixels), Some(renderer)) = (&self.pixels, &self.renderer) {
            let gpu_buf = renderer.render_to_image(
                pixels.device(),
                pixels.queue(),
                &self.mandelbrot,
                width,
                height,
                use_perturbation,
                pixels.render_texture_format(),
            );
            let img = image::RgbaImage::from_raw(width, height, gpu_buf).unwrap();
            img.save(format!("{dir}/gpu_render.png")).unwrap();
        }

        eprintln!("Debug dump saved to {dir}/");
    }

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

            let perturbation_str = if self.mandelbrot.perturbation_active {
                " | Perturbation"
            } else {
                ""
            };

            if self.gpu_rendering {
                window.set_title(&format!("rsfractal | (M)ode: {renderer} | (C)oloring: {coloring} | Iterations(↑↓): {iterations}{perturbation_str} | {fps:.1} fps"));
            } else {
                let rendering = &self.mandelbrot.rendering;
                window.set_title(&format!("rsfractal | (M)ode: {renderer} | (R)endering: {rendering} | (C)oloring: {coloring} | Iterations(↑↓): {iterations}{perturbation_str} | {fps:.1} fps"));
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
        if let Some(window) = &self.window
            && let DeviceEvent::MouseMotion { delta } = event
            && self.mouse_button
        {
            self.mandelbrot.pan(delta.0, delta.1);
            window.request_redraw();
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
                            self.mandelbrot
                                .set_resolution(size.width as usize, size.height as usize);
                            let _ = pixels.resize_buffer(size.width, size.height);
                        }
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key {
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
                    if !self.gpu_rendering
                        && let (Some(window), Some(pixels)) = (&self.window, &mut self.pixels)
                    {
                        let size = window.inner_size();
                        self.mandelbrot
                            .set_resolution(size.width as usize, size.height as usize);
                        let _ = pixels.resize_buffer(size.width, size.height);
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
                    self.mandelbrot.selected_palette =
                        (self.mandelbrot.selected_palette + 1) % self.mandelbrot.palettes().len();
                    if let (Some(pixels), Some(renderer)) = (&self.pixels, &mut self.renderer) {
                        renderer.update_coloring(pixels.device(), pixels.queue(), &self.mandelbrot);
                    }
                    self.update_title();
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                KeyCode::KeyD => {
                    self.debug_dump();
                }
                KeyCode::ArrowUp => {
                    self.mandelbrot.max_iterations = (self.mandelbrot.max_iterations * 2).min(100000);
                    self.mandelbrot.perturbation_dirty = true;
                    self.update_title();
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                KeyCode::ArrowDown => {
                    self.mandelbrot.max_iterations = (self.mandelbrot.max_iterations / 2).max(10);
                    self.mandelbrot.perturbation_dirty = true;
                    self.update_title();
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
                _ => (),
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = (position.x, position.y);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let zoom_factor = match delta {
                    MouseScrollDelta::PixelDelta(pos) => {
                        if pos.y == 0.0 { return; }
                        // Trackpad: proportional to gesture magnitude
                        f64::powf(0.997, pos.y)
                    }
                    MouseScrollDelta::LineDelta(_, y) => {
                        if y == 0.0 { return; }
                        // Mouse wheel: fixed step per click
                        if y > 0.0 { 0.9 } else { 1.1 }
                    }
                };
                if let Some(window) = &self.window {
                    let size = window.inner_size();
                    let (cx, cy) = self.cursor_position;

                    // Update HP position before modifying zoom
                    self.mandelbrot.zoom_hp(
                        cx / size.width as f64,
                        cy / size.height as f64,
                        zoom_factor as f64,
                    );

                    // Compute zoom-toward-cursor in f64 to avoid f32 cancellation at deep zoom.
                    // In f32, position ± zoom collapses to position once zoom < ULP(position),
                    // making Range::scale map every cursor position to the center.
                    let cursor_frac_x = cx / size.width as f64;
                    let cursor_frac_y = cy / size.height as f64;
                    let pos_re = self.mandelbrot.position.x as f64;
                    let pos_im = self.mandelbrot.position.y as f64;
                    let zoom_re = self.mandelbrot.zoom.x;
                    let zoom_im = self.mandelbrot.zoom.y;
                    let zf = zoom_factor as f64;

                    // Complex coordinate under cursor
                    let target_re = pos_re + (2.0 * cursor_frac_x - 1.0) * zoom_re;
                    let target_im = pos_im + (2.0 * cursor_frac_y - 1.0) * zoom_im;

                    self.mandelbrot.zoom.x = zoom_re * zf;
                    self.mandelbrot.zoom.y = zoom_im * zf;
                    self.mandelbrot.position.x = (target_re + (pos_re - target_re) * zf) as f32;
                    self.mandelbrot.position.y = (target_im + (pos_im - target_im) * zf) as f32;

                    // When HP position is available, sync f32 position from it —
                    // HP accumulates sub-f64 offsets that the f64 computation above loses.
                    if let Some(hp) = &self.mandelbrot.high_precision_position {
                        self.mandelbrot.position.x = hp.x.to_f64().value() as f32;
                        self.mandelbrot.position.y = hp.y.to_f64().value() as f32;
                    }

                    window.request_redraw();
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

                // Update perturbation state before rendering
                {
                    let was_dirty = self.mandelbrot.perturbation_dirty;
                    let was_active = self.mandelbrot.perturbation_active;
                    self.mandelbrot.update_perturbation_state();
                    // Only re-upload the orbit buffer when something actually changed
                    if self.mandelbrot.perturbation_active && (!was_active || was_dirty) {
                        if let Some(orbit) = self.mandelbrot.reference_orbit.clone() {
                            if let (Some(pixels), Some(renderer)) =
                                (&self.pixels, &mut self.renderer)
                            {
                                renderer.update_perturbation(pixels.device(), &orbit);
                            }
                        }
                    }
                }

                let use_perturbation = self.mandelbrot.perturbation_active;

                if let Some(pixels) = &mut self.pixels {
                    if self.gpu_rendering {
                        if let (Some(renderer), Some(window)) = (&self.renderer, &self.window) {
                            let size = window.inner_size();
                            if use_perturbation {
                                pixels
                                    .render_with(|encoder, render_target, context| {
                                        renderer.set_perturbation_params(
                                            &context.queue,
                                            &self.mandelbrot,
                                            size.width as f32,
                                            size.height as f32,
                                        );
                                        renderer.render_perturbation(
                                            encoder,
                                            render_target,
                                            (0, 0, size.width, size.height),
                                        );
                                        Ok(())
                                    })
                                    .unwrap();
                            } else {
                                pixels
                                    .render_with(|encoder, render_target, context| {
                                        renderer.set_params(
                                            &context.queue,
                                            &self.mandelbrot,
                                            size.width as f32,
                                            size.height as f32,
                                        );
                                        renderer.render(
                                            encoder,
                                            render_target,
                                            (0, 0, size.width, size.height),
                                        );
                                        Ok(())
                                    })
                                    .unwrap();
                            }
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
    let mut app = App {
        gpu_rendering: true,
        ..Default::default()
    };
    event_loop.run_app(&mut app)?;
    Ok(())
}
