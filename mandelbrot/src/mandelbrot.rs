use color::DynamicColor;
use color::HueDirection;
use color::Lch;
use color::OpaqueColor;
use color::Rgba8;
use color::parse_color;
use num_complex::Complex;
use num_traits::MulAdd;
use num_traits::Num;
use num_traits::Zero;
use rayon::prelude::*;

use super::range::Range;
use super::rectangle::Rectangle;
use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub position: Vector<f64>,
    pub zoom: Vector<f64>,
    pub iterations: usize,
    pub palette: Vec<DynamicColor>,
}

impl Config {
    pub fn zoom(&mut self, x: f64, y: f64, zoom_factor: f64) {
        let width_range = Range::new(0.0, self.width as f64);
        let height_range = Range::new(0.0, self.height as f64);
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

    pub fn palette(&self, iterations: f64) -> Rgba8 {
        let color1 = self.palette[f64::floor(iterations) as usize % self.palette.len()];
        let color2 = self.palette[f64::floor(iterations + 1.0) as usize % self.palette.len()];
        let i = color1.interpolate(color2, color::ColorSpaceTag::Lch, HueDirection::default());
        i.eval(iterations as f32 % 1.0).to_alpha_color::<Lch>().to_rgba8()
    }

    pub fn lch(&self, iterations: f64) -> Rgba8 {
        let s = iterations as f32 / self.iterations as f32;
        let v = 1.0 - f32::powf(f32::cos(std::f32::consts::PI * s), 2.0);
        OpaqueColor::<Lch>::new([
            75.0 - (75.0 * v),
            28.0 + (75.0 - (75.0 * v)),
            f32::powf(360.0 * s, 1.5) % 360.0,
        ])
        .to_rgba8()
    }
}

impl Default for Config {
    fn default() -> Self {
        let palette = ["#3e0000", "#6b1d09", "#9a542e", "#bf935c", "#d0c8a8"];
        let palette: Vec<DynamicColor> = palette.iter().map(|hex| parse_color(hex).unwrap()).collect();

        Self {
            width: 1280,
            height: 720,
            position: Vector { x: -0.5, y: 0.0 },
            zoom: Vector { x: 2.0, y: 1.125 },
            iterations: 1000,
            palette,
        }
    }
}

pub fn render(config: &Config, pixels: &mut [u8]) {
    let width_range = Range::new(0.0, config.width as f64);
    let height_range = Range::new(0.0, config.height as f64);

    let rect = rect_from_position(&config.position, &config.zoom);
    let real_range = Range::new(rect.start.x, rect.end.x);
    let imaginary_range = Range::new(rect.start.y, rect.end.y);

    let chunk_size = pixels.len() / rayon::current_num_threads();
    pixels
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(index, pixels)| {
            let start = index * chunk_size / 4;
            pixels.chunks_mut(4).enumerate().for_each(|(index, pixel)| {
                let x = ((start + index) % config.width as usize) as u32;
                let y = ((start + index) / config.width as usize) as u32;

                let c = Complex::new(
                    Range::scale(&width_range, x as f64, &real_range),
                    Range::scale(&height_range, y as f64, &imaginary_range),
                );

                let iterations = iterate(config.iterations, &c);
                if iterations < config.iterations as f64 {
                    let color = config.palette(iterations);
                    pixel.copy_from_slice(&color.to_u8_array());
                } else {
                    pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
                }
            });
        })
}

fn iterate(max_iterations: usize, c: &Complex<f64>) -> f64 {
    let mut z: Complex<f64> = Complex::zero();

    let im2 = c.im * c.im;
    let mut q = c.re - 0.25;
    q *= q;
    q += im2;

    if q * (q + (c.re - 0.25)) < 0.25 * im2 {
        max_iterations as f64
    } else {
        let mut iterations = 0;
        while z.norm_sqr() < 65536.0 && iterations < max_iterations {
            let temp = z.mul_add(z, *c);
            if z == temp {
                return max_iterations as f64;
            }
            z = temp;
            iterations += 1;
        }
        if iterations < max_iterations {
            let zn = f64::log10(z.norm_sqr()) / 2.0;
            let nu = f64::log10(zn / std::f64::consts::LOG10_2) / std::f64::consts::LOG10_2;
            return (iterations + 1) as f64 - nu;
        }
        iterations as f64
    }
}

pub fn rect_from_position<T: Num + Copy>(position: &Vector<T>, zoom: &Vector<T>) -> Rectangle<T> {
    Rectangle::new(
        Vector::new(position.x - zoom.x, position.y - zoom.y),
        Vector::new(position.x + zoom.x, position.y + zoom.y),
    )
}
