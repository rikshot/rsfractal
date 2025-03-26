use std::fmt::Debug;

use colorgrad::{CatmullRomGradient, Color, Gradient, GradientBuilder};
use num::complex::Complex;
use num::traits::MulAddAssign;
use rayon::prelude::*;
use strum::{Display, EnumIter, EnumString};

use crate::boundary_scanner::BoundaryScanner;

use super::range::Range;
use super::rectangle::Rectangle;
use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    resolutions: Vec<(usize, usize)>,
    pub selected_resolution: usize,
    pub(crate) position: Vector,
    pub(crate) zoom: Vector,
    pub rendering: Rendering,
    pub bailout: f64,
    pub max_iterations: usize,
    pub chunk_size: usize,
    pub period_length: usize,
    pub coloring: Coloring,
    pub exponent: f64,
    pub(crate) palettes: Vec<(String, CatmullRomGradient)>,
    pub selected_palette: usize,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, EnumIter)]
pub enum Rendering {
    Smooth,
    Approximate,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, EnumIter)]
pub enum Coloring {
    Palette,
    LCH,
}

pub(crate) fn rect_from_position(position: &Vector, zoom: &Vector) -> Rectangle {
    Rectangle::new(
        Vector::new(position.x - zoom.x, position.y - zoom.y),
        Vector::new(position.x + zoom.x, position.y + zoom.y),
    )
}

impl Mandelbrot {
    pub fn width(&self) -> usize {
        self.resolutions[self.selected_resolution].0
    }

    pub fn height(&self) -> usize {
        self.resolutions[self.selected_resolution].1
    }

    pub fn resolutions(&self) -> &[(usize, usize)] {
        &self.resolutions
    }

    pub fn palettes(&self) -> &[(String, CatmullRomGradient)] {
        &self.palettes
    }

    pub fn render(&self, pixels: &mut [u8]) {
        match self.rendering {
            Rendering::Smooth => self.render_smooth(pixels),
            Rendering::Approximate => self.render_fast(pixels),
        }
    }

    fn render_smooth(&self, pixels: &mut [u8]) {
        let width_range = Range::new(0.0, self.width() as f64);
        let height_range = Range::new(0.0, self.height() as f64);

        let rect = rect_from_position(&self.position, &self.zoom);
        let real_range = Range::new(rect.start.x, rect.end.x);
        let imaginary_range = Range::new(rect.start.y, rect.end.y);

        let chunk_size = pixels.len() / rayon::current_num_threads();
        pixels
            .par_chunks_exact_mut(chunk_size)
            .enumerate()
            .for_each(|(index, pixels)| {
                let start = index * chunk_size / 4;
                pixels.chunks_exact_mut(4).enumerate().for_each(|(index, pixel)| {
                    let x = ((start + index) % self.width()) as f64;
                    let y = ((start + index) / self.width()) as f64;

                    let c = Complex::new(
                        Range::scale(&width_range, x, &real_range),
                        Range::scale(&height_range, y, &imaginary_range),
                    );

                    let (z, iterations) = self.iterate(&c);
                    if iterations < self.max_iterations {
                        let color = match self.coloring {
                            Coloring::Palette => self.palette(Some(&z), iterations),
                            Coloring::LCH => self.lch(Some(&z), iterations),
                        };
                        pixel.copy_from_slice(&color.to_rgba8());
                    } else {
                        pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
                    }
                });
            })
    }

    fn render_fast(&self, pixels: &mut [u8]) {
        let rows = self.height() / rayon::current_num_threads();
        let chunk_size = self.width() * rows * 4;
        pixels
            .par_chunks_exact_mut(chunk_size)
            .enumerate()
            .for_each(|(index, pixels)| {
                let start = index * rows;
                let mut boundary_scanner = BoundaryScanner::new(self, start, start + rows);
                let data = boundary_scanner.run();
                pixels.chunks_exact_mut(4).enumerate().for_each(|(index, pixel)| {
                    let iterations = data[index];
                    if iterations < self.max_iterations {
                        let color = match self.coloring {
                            Coloring::Palette => self.palette(None, iterations),
                            Coloring::LCH => self.lch(None, iterations),
                        };
                        pixel.copy_from_slice(&color.to_rgba8());
                    } else {
                        pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
                    }
                });
            });
    }

    pub(crate) fn iterate(&self, c: &Complex<f64>) -> (Complex<f64>, usize) {
        let mut z: Complex<f64> = Complex::ZERO;

        let im2 = c.im * c.im;
        let mut q = c.re - 0.25;
        q *= q;
        q += im2;

        if q * (q + (c.re - 0.25)) < 0.25 * im2 {
            (z, self.max_iterations)
        } else {
            let mut iterations = 0;
            let mut old: Complex<f64> = Complex::ZERO;
            let mut period = 0;
            while z.norm_sqr() < self.bailout && iterations < self.max_iterations {
                z.mul_add_assign(z, *c);
                if z == old {
                    return (z, self.max_iterations);
                }
                iterations += 1;
                period += 1;
                if period > self.period_length {
                    period = 0;
                    old = z;
                }
            }
            (z, iterations)
        }
    }

    pub fn zoom(&mut self, x: f64, y: f64, zoom_factor: f64) {
        let width_range = Range::new(0.0, self.width() as f64);
        let height_range = Range::new(0.0, self.height() as f64);
        let selection = rect_from_position(&self.position, &self.zoom);
        let real_range = Range::new(selection.start.x, selection.end.x);
        let imaginary_range = Range::new(selection.start.y, selection.end.y);
        self.position = Vector {
            x: Range::scale(&width_range, x, &real_range),
            y: Range::scale(&height_range, y, &imaginary_range),
        };
        self.zoom = Vector {
            x: self.zoom.x * zoom_factor,
            y: self.zoom.y * zoom_factor,
        };
    }

    fn smooth(&self, z: &Complex<f64>, iterations: usize) -> f64 {
        if iterations < self.max_iterations {
            let zn = f64::ln(z.norm_sqr()) / 2.0;
            let nu = f64::ln(zn / std::f64::consts::LN_2) / std::f64::consts::LN_2;
            return (iterations + 1) as f64 - nu;
        }
        iterations as f64
    }

    fn exponential(&self, iterations: f64) -> f64 {
        f64::powf(f64::powf(iterations / self.max_iterations as f64, self.exponent), 1.5)
    }

    fn palette(&self, z: Option<&Complex<f64>>, iterations: usize) -> Color {
        let smooth = if let Some(z) = z {
            self.smooth(z, iterations)
        } else {
            iterations as f64
        };
        let exponential = self.exponential(smooth);
        let (_, palette) = &self.palettes[self.selected_palette];
        palette.at(exponential as f32)
    }

    fn lch(&self, z: Option<&Complex<f64>>, iterations: usize) -> Color {
        let smooth = if let Some(z) = z {
            self.smooth(z, iterations)
        } else {
            iterations as f64
        };
        let s = self.exponential(smooth);
        let v = 1.0 - f64::powf(f64::cos(std::f64::consts::PI * s), 2.0);
        Color::from_lcha(
            (75.0 - (75.0 * v)) as f32,
            (28.0 + (75.0 - (75.0 * v))) as f32,
            (f64::powf(360.0 * s, 1.5) % 360.0) as f32,
            1.0,
        )
    }
}

impl Default for Mandelbrot {
    fn default() -> Self {
        let resolutions = vec![
            (320, 180),
            (640, 360),
            (960, 540),
            (1280, 720),
            (1600, 900),
            (1920, 1080),
            (3840, 2160),
        ];

        let palettes = [
            ("Rust", vec!["#3e0000", "#6b1d09", "#9a542e", "#bf935c", "#d0c8a8"]),
            ("Cold Teal", vec!["#E3FDFD", "#CBF1F5", "#A6E3E9", "#71C9CE"]),
            ("Sunset", vec!["#F9ED69", "#F08A5D", "#B83B5E", "#6A2C70"]),
        ];
        let palettes: Vec<(String, CatmullRomGradient)> = palettes
            .into_iter()
            .map(|(name, hex_list)| {
                (
                    name.to_string(),
                    GradientBuilder::new()
                        .html_colors(&hex_list)
                        .mode(colorgrad::BlendMode::Oklab)
                        .build::<CatmullRomGradient>()
                        .unwrap(),
                )
            })
            .collect();

        Self {
            resolutions,
            selected_resolution: if cfg!(debug_assertions) { 0 } else { 5 },
            position: Vector { x: -0.5, y: 0.0 },
            zoom: Vector { x: 2.0, y: 1.125 },
            rendering: Rendering::Approximate,
            bailout: f64::powf(2.0, 16.0),
            max_iterations: 100000,
            chunk_size: usize::pow(2, 21),
            period_length: 20,
            coloring: Coloring::LCH,
            exponent: 1.0,
            palettes,
            selected_palette: 0,
        }
    }
}
