use num_complex::Complex;
use num_traits::Num;
use num_traits::Zero;

use serde::{Deserialize, Serialize};

use super::color::Color;
use super::range::Range;
use super::rectangle::Rectangle;
use super::vector::Vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip)]
    pub width: u32,
    #[serde(skip)]
    pub height: u32,
    pub position: Vector<f64>,
    pub zoom: Vector<f64>,
    pub iterations: u32,
    pub palette: Vec<Color>,
    #[serde(skip)]
    pub chunk_size: Option<u32>,
}

impl Default for Config {
    fn default() -> Self {
        let palette = ["#3e0000", "#6b1d09", "#9a542e", "#bf935c", "#d0c8a8"].repeat(2);
        let palette: Vec<Color> = palette.iter().map(|hex| Color::from_hex(hex).unwrap()).collect();

        Self {
            width: 1280,
            height: 720,
            position: Vector { x: -0.5, y: 0.0 },
            zoom: Vector { x: 2.0, y: 1.125 },
            iterations: 1000,
            chunk_size: Some(128),
            palette,
        }
    }
}

#[derive(Clone)]
pub struct ChunkConfig {
    pub screen: Rectangle<u32>,
    complex: Rectangle<f64>,
}

pub struct ChunkResult {
    pub iterations: Vec<u32>,
    pub fractional: Vec<Option<f64>>,
    pub histogram: Vec<u32>,
    pub total: u32,
}

pub fn iterate(config: &Config, chunk_config: &ChunkConfig) -> ChunkResult {
    let size = (chunk_config.screen.width() * chunk_config.screen.height()) as usize;
    let ln2 = std::f64::consts::LN_2;

    let mut histogram: Vec<u32> = vec![0; config.iterations as usize];
    let mut iterations: Vec<u32> = vec![0; size];
    let mut fractional: Vec<Option<f64>> = vec![None; size];

    let width_range = Range::new(chunk_config.screen.start.x as f64, chunk_config.screen.end.x as f64);
    let height_range = Range::new(chunk_config.screen.start.y as f64, chunk_config.screen.end.y as f64);
    let real_range = Range::new(chunk_config.complex.start.x, chunk_config.complex.end.x);
    let imaginary_range = Range::new(chunk_config.complex.start.y, chunk_config.complex.end.y);

    let mut total = 0;
    let mut index = 0;
    for y in chunk_config.screen.start.y..chunk_config.screen.end.y {
        for x in chunk_config.screen.start.x..chunk_config.screen.end.x {
            let mut z: Complex<f64> = Complex::zero();
            let c = Complex::new(
                Range::scale(&width_range, x as f64, &real_range),
                Range::scale(&height_range, y as f64, &imaginary_range),
            );

            let im2 = c.im * c.im;
            let mut q = c.re - 0.25;
            q *= q;
            q += im2;

            if q * (q + (c.re - 0.25)) < 0.25 * im2 {
                iterations[index] = config.iterations;
            } else {
                let mut iteration = 0;
                while z.norm_sqr() < (1 << 16) as f64 && iteration < config.iterations {
                    let temp = z * z + c;
                    if z == temp {
                        iteration = config.iterations;
                        break;
                    }
                    z = temp;
                    iteration += 1;
                }
                iterations[index] = iteration;
                if iteration < config.iterations {
                    fractional[index] = Some(iteration as f64 + 1.0 - f64::ln(f64::ln(z.norm_sqr()) / 2.0 / ln2) / ln2);
                    histogram[iteration as usize] += 1;
                    total += 1;
                }
            }
            index += 1;
        }
    }
    ChunkResult {
        iterations,
        fractional,
        histogram,
        total,
    }
}

pub fn iterate_single(max_iterations: u32, c: &Complex<f64>) -> f64 {
    let mut z: Complex<f64> = Complex::zero();

    let im2 = c.im * c.im;
    let mut q = c.re - 0.25;
    q *= q;
    q += im2;

    if q * (q + (c.re - 0.25)) < 0.25 * im2 {
        max_iterations as f64
    } else {
        let mut iteration = 0;
        while z.norm_sqr() < (1 << 16) as f64 && iteration < max_iterations {
            let temp = z * z + c;
            if z == temp {
                return max_iterations as f64;
            }
            z = temp;
            iteration += 1;
        }
        if iteration < max_iterations {
            let log_zn = f64::ln(z.norm_sqr()) / 2.0;
            let nu = f64::ln(log_zn / std::f64::consts::LN_2) / std::f64::consts::LN_2;
            return iteration as f64 + 1.0 - nu;
        }
        iteration as f64
    }
}

fn gradient(config: &Config, hue1: f64, hue2: f64, n: f64) -> Color {
    let length = (config.palette.len() - 1) as f64;
    let color1 = &config.palette[f64::round(hue1 * length) as usize];
    let color2 = &config.palette[f64::round(hue2 * length) as usize];
    Color::lerp(color1, color2, n)
}

pub fn color(
    config: &Config,
    chunk_config: &ChunkConfig,
    result: &ChunkResult,
    histogram: &[u32],
    total: u32,
) -> Vec<Color> {
    let size = chunk_config.screen.width() * chunk_config.screen.height();
    let mut pixels = Vec::new();
    for index in 0..size as usize {
        if let Some(fractional) = result.fractional[index] {
            let iteration = result.iterations[index];
            let mut hue = 0.0;
            for i in histogram.iter().take(iteration as usize) {
                hue += *i as f64 / total as f64;
            }
            let color = gradient(
                config,
                hue,
                hue + histogram[iteration as usize] as f64 / total as f64,
                fractional % 1.0,
            );
            pixels.push(color);
        } else {
            pixels.push(Color::new(0, 0, 0));
        }
    }
    pixels
}

pub fn rect_from_position<T: Num + Copy>(position: &Vector<T>, zoom: &Vector<T>) -> Rectangle<T> {
    Rectangle::new(
        Vector::new(position.x - zoom.x, position.y - zoom.y),
        Vector::new(position.x + zoom.x, position.y + zoom.y),
    )
}

pub fn chunkify(config: &Config) -> Vec<ChunkConfig> {
    let size = config.chunk_size.unwrap_or(512);

    let width_range = Range::new(0.0, config.width as f64);
    let height_range = Range::new(0.0, config.height as f64);

    let selection = rect_from_position(&config.position, &config.zoom);

    let real_range = Range::new(selection.start.x as f64, selection.end.x as f64);
    let imaginary_range = Range::new(selection.start.y as f64, selection.end.y as f64);

    let mut chunks: Vec<ChunkConfig> = Vec::new();
    for x in (0..config.width).step_by(size as usize) {
        let chunk_width = if x + size > config.width {
            config.width - x
        } else {
            size
        };
        for y in (0..config.height).step_by(size as usize) {
            let chunk_height = if y + size > config.height {
                config.height - y
            } else {
                size
            };
            let screen_start = Vector::new(x, y);
            let screen_end = Vector::new(x + chunk_width, y + chunk_height);
            chunks.push(ChunkConfig {
                screen: Rectangle::new(screen_start, screen_end),
                complex: Rectangle::new(
                    Vector::new(
                        Range::scale(&width_range, x as f64, &real_range),
                        Range::scale(&height_range, y as f64, &imaginary_range),
                    ),
                    Vector::new(
                        Range::scale(&width_range, (x + chunk_width) as f64, &real_range),
                        Range::scale(&height_range, (y + chunk_height) as f64, &imaginary_range),
                    ),
                ),
            })
        }
    }
    chunks
}

pub fn zoom(config: &mut Config, x: f64, y: f64, width: f64, height: f64, zoom_factor: f64) {
    let width_range = Range::new(0.0, width);
    let height_range = Range::new(0.0, height);
    let selection = rect_from_position(&config.position, &config.zoom);
    let real_range = Range::new(selection.start.x, selection.end.x);
    let imaginary_range = Range::new(selection.start.y, selection.end.y);
    config.position = Vector {
        x: Range::scale(&width_range, x, &real_range),
        y: Range::scale(&height_range, y, &imaginary_range),
    };
    config.zoom = Vector {
        x: config.zoom.x * zoom_factor,
        y: config.zoom.y * zoom_factor,
    };
}
