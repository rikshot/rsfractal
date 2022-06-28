use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use rayon::prelude::*;

use rsfractal_mandelbrot::color::Color;
use rsfractal_mandelbrot::mandelbrot::{chunkify, color, iterate, iterate_single, rect_from_position, zoom, Config};
use rsfractal_mandelbrot::range::Range;

use num_complex::Complex;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("rsfractal")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut config = Config::default();
    let mut mouse_position = None;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => pixels.resize_surface(size.width, size.height),
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                mouse_position = Some(position);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        button: MouseButton::Left,
                        state: ElementState::Released,
                        ..
                    },
                ..
            } => {
                if let Some(mouse_position) = mouse_position {
                    let (x, y) = pixels.window_pos_to_pixel(mouse_position.into()).unwrap();
                    zoom(&mut config, x as f64, y as f64, WIDTH as f64, HEIGHT as f64, 0.25);
                    window.request_redraw()
                }
            }
            Event::RedrawRequested(_) => {
                render_alt(&config, pixels.get_frame());
                pixels.render().unwrap();
            }
            _ => (),
        }
    });
}

fn render(config: &Config, pixels: &mut [u8]) {
    let chunks = chunkify(config);
    let results: Vec<_> = chunks.par_iter().map(|chunk| iterate(config, chunk)).collect();
    let (histogram, total) = results.iter().fold(
        (vec![0; config.iterations as usize], 0),
        |(histogram, total), result| {
            (
                result
                    .histogram
                    .iter()
                    .enumerate()
                    .map(|(index, iterations)| histogram[index] + iterations)
                    .collect(),
                total + result.total,
            )
        },
    );
    let colors: Vec<_> = chunks
        .par_iter()
        .zip(&results)
        .map(|(chunk, result)| color(config, chunk, result, &histogram, total))
        .collect();
    chunks.iter().zip(&colors).for_each(|(chunk, colors)| {
        let mut index = 0;
        for y in chunk.screen.start.y..chunk.screen.end.y {
            let stride = y * WIDTH * 4;
            for x in chunk.screen.start.x..chunk.screen.end.x {
                let pixel = &colors[index];
                let stride = (stride + x * 4) as usize;
                pixels[stride] = pixel.r;
                pixels[stride + 1] = pixel.g;
                pixels[stride + 2] = pixel.b;
                pixels[stride + 3] = 0xFF;
                index += 1;
            }
        }
    })
}

fn render_alt(config: &Config, pixels: &mut [u8]) {
    let width_range = Range::new(0.0, config.width as f64);
    let height_range = Range::new(0.0, config.height as f64);

    let rect = rect_from_position(&config.position, &config.zoom);
    let real_range = Range::new(rect.start.x, rect.end.x);
    let imaginary_range = Range::new(rect.start.y, rect.end.y);

    pixels.par_chunks_mut(4).enumerate().for_each(|(index, pixel)| {
        let x = (index % WIDTH as usize) as u32;
        let y = (index / WIDTH as usize) as u32;

        let c = Complex::new(
            Range::scale(&width_range, x as f64, &real_range),
            Range::scale(&height_range, y as f64, &imaginary_range),
        );

        let iterations = iterate_single(config.iterations, &c);
        if iterations < config.iterations as f64 {
            let color1 = config.palette[f64::floor(iterations) as usize % config.palette.len()];
            let color2 = config.palette[(f64::floor(iterations) as usize + 1) % config.palette.len()];
            let color = Color::lerp(&color1, &color2, iterations % 1.0);
            pixel.copy_from_slice(&[color.r, color.g, color.b, 0xFF]);
        } else {
            pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
        }
    })
}
