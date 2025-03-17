use color::DynamicColor;
use color::HueDirection;
use color::Lch;
use color::OpaqueColor;
use color::Rgba8;
use color::parse_color;
use num_complex::Complex;
use rayon::prelude::*;
use strum::Display;
use strum::EnumIter;
use strum::EnumString;

use super::range::Range;
use super::rectangle::Rectangle;
use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Mandelbrot {
    pub width: u32,
    pub height: u32,
    position: Vector,
    zoom: Vector,
    pub bailout: f64,
    pub max_iterations: usize,
    pub exponent: f64,
    pub coloring: Coloring,
    pub palettes: Vec<(String, Vec<DynamicColor>)>,
    pub selected_palette: usize,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, EnumIter)]
pub enum Coloring {
    Palette,
    LCH,
}

impl Mandelbrot {
    pub fn render(&self, pixels: &mut [u8]) {
        let width_range = Range::new(0.0, self.width as f64);
        let height_range = Range::new(0.0, self.height as f64);

        let rect = self.rect_from_position();
        let real_range = Range::new(rect.start.x, rect.end.x);
        let imaginary_range = Range::new(rect.start.y, rect.end.y);

        let chunk_size = pixels.len() / rayon::current_num_threads();
        pixels
            .par_chunks_exact_mut(chunk_size)
            .enumerate()
            .for_each(|(index, pixels)| {
                let start = index * chunk_size / 4;
                pixels.chunks_exact_mut(4).enumerate().for_each(|(index, pixel)| {
                    let x = ((start + index) % self.width as usize) as f64;
                    let y = ((start + index) / self.width as usize) as f64;

                    let c = Complex::new(
                        Range::scale(&width_range, x, &real_range),
                        Range::scale(&height_range, y, &imaginary_range),
                    );

                    let (z, iterations) = self.iterate(&c);
                    if iterations < self.max_iterations {
                        let color = match self.coloring {
                            Coloring::Palette => self.palette(&z, iterations),
                            Coloring::LCH => self.lch(&z, iterations),
                        };
                        pixel.copy_from_slice(&color.to_u8_array());
                    } else {
                        pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
                    }
                });
            })
    }

    fn iterate(&self, c: &Complex<f64>) -> (Complex<f64>, usize) {
        let mut z: Complex<f64> = Complex::ZERO;

        let im2 = c.im * c.im;
        let mut q = c.re - 0.25;
        q *= q;
        q += im2;

        if q * (q + (c.re - 0.25)) < 0.25 * im2 {
            (z, self.max_iterations)
        } else {
            let mut iterations = 0;
            while z.norm_sqr() < self.bailout && iterations < self.max_iterations {
                let temp = z * z + *c;
                if z == temp {
                    return (z, self.max_iterations);
                }
                z = temp;
                iterations += 1;
            }
            (z, iterations)
        }
    }

    fn rect_from_position(&self) -> Rectangle {
        Rectangle::new(
            Vector::new(self.position.x - self.zoom.x, self.position.y - self.zoom.y),
            Vector::new(self.position.x + self.zoom.x, self.position.y + self.zoom.y),
        )
    }

    pub fn zoom(&mut self, x: f64, y: f64, zoom_factor: f64) {
        let width_range = Range::new(0.0, self.width as f64);
        let height_range = Range::new(0.0, self.height as f64);
        let selection = self.rect_from_position();
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
        f64::powf(
            f64::powf(iterations as f64 / self.max_iterations as f64, self.exponent),
            1.5,
        )
    }

    fn exponential_cyclic(&self, iterations: f64, n: usize) -> usize {
        (self.exponential(iterations) * n as f64) as usize % n
    }

    fn palette(&self, z: &Complex<f64>, iterations: usize) -> Rgba8 {
        let (_, palette) = &self.palettes[self.selected_palette];
        let smooth = self.smooth(z, iterations);
        let color1 = palette[self.exponential_cyclic(f64::floor(smooth), palette.len())];
        let color2 = palette[self.exponential_cyclic(f64::ceil(smooth), palette.len())];
        let interpolator = color1.interpolate(color2, color::ColorSpaceTag::Lch, HueDirection::default());
        interpolator
            .eval(smooth as f32 % 1.0)
            .to_alpha_color::<Lch>()
            .to_rgba8()
    }

    fn lch(&self, z: &Complex<f64>, iterations: usize) -> Rgba8 {
        let s = self.smooth(z, iterations);
        let s = self.exponential(s);
        let v = 1.0 - f64::powf(f64::cos(std::f64::consts::PI * s), 2.0);
        OpaqueColor::<Lch>::new([
            (75.0 - (75.0 * v)) as f32,
            (28.0 + (75.0 - (75.0 * v))) as f32,
            (f64::powf(360.0 * s, 1.5) % 360.0) as f32,
        ])
        .to_rgba8()
    }
}

impl Default for Mandelbrot {
    fn default() -> Self {
        let palettes = [
            ("Rust", vec!["#3e0000", "#6b1d09", "#9a542e", "#bf935c", "#d0c8a8"]),
            ("Cold Teal", vec!["#E3FDFD", "#CBF1F5", "#A6E3E9", "#71C9CE"]),
            ("Sunset", vec!["#F9ED69", "#F08A5D", "#B83B5E", "#6A2C70"]),
        ];
        let palettes: Vec<(String, Vec<DynamicColor>)> = palettes
            .into_iter()
            .map(|(name, hex_list)| {
                (
                    name.to_string(),
                    hex_list.into_iter().map(|hex| parse_color(hex).unwrap()).collect(),
                )
            })
            .collect();

        Self {
            width: if cfg!(debug_assertions) { 320 } else { 1280 },
            height: if cfg!(debug_assertions) { 180 } else { 720 },
            position: Vector { x: -0.5, y: 0.0 },
            zoom: Vector { x: 2.0, y: 1.125 },
            bailout: f64::powf(2.0, 32.0),
            max_iterations: 1000,
            exponent: 0.25,
            coloring: Coloring::Palette,
            palettes,
            selected_palette: 0,
        }
    }
}
