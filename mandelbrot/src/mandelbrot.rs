use std::fmt::Debug;
use std::sync::Arc;

use colorgrad::{CatmullRomGradient, Color, Gradient, GradientBuilder};
use num::complex::Complex32;
use rayon::prelude::*;
use serde::Serialize;
use strum::{Display, EnumIter, EnumString};

use crate::boundary_scanner::BoundaryScanner;
use crate::perturbation::{
    compute_reference_orbit_with_sa, iterate_perturbation, needs_perturbation, recompute_sa,
    required_precision_bits, test_orbit_f64, HighPrecisionComplex, HighPrecisionPosition,
    ReferenceOrbit,
};

use super::range::Range;
use super::rectangle::Rectangle;
use super::vector::Vector;

#[derive(Debug, Clone, Serialize)]
pub struct Mandelbrot {
    pub width: usize,
    pub height: usize,
    pub position: Vector,
    pub zoom: Vector<f64>,
    pub rendering: Rendering,
    pub bailout: f32,
    pub max_iterations: usize,
    pub chunk_size: usize,
    pub period_length: usize,
    pub coloring: Coloring,
    pub exponent: f32,
    #[serde(skip)]
    pub(crate) palettes: Vec<(String, CatmullRomGradient)>,
    pub selected_palette: usize,
    pub high_precision_position: Option<HighPrecisionPosition>,
    pub reference_orbit: Option<Arc<ReferenceOrbit>>,
    pub perturbation_active: bool,
    pub reference_offset: (f64, f64),
    pub reference_point: Option<HighPrecisionComplex>,
    pub perturbation_dirty: bool,
    pub sa_order: usize,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, EnumIter, Serialize)]
pub enum Rendering {
    Smooth,
    Fast,
}

#[derive(Debug, Clone, PartialEq, Display, EnumString, EnumIter, Serialize)]
pub enum Coloring {
    Palette,
    LCH,
}

pub fn rect_from_position(position: &Vector, zoom: &Vector<f64>) -> Rectangle {
    Rectangle::new(
        Vector::new(position.x - zoom.x as f32, position.y - zoom.y as f32),
        Vector::new(position.x + zoom.x as f32, position.y + zoom.y as f32),
    )
}

impl Mandelbrot {
    pub fn set_resolution(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn palettes(&self) -> &[(String, CatmullRomGradient)] {
        &self.palettes
    }

    pub fn ranges(&self) -> [Range; 4] {
        let width_range = Range::new(0.0, self.width as f32);
        let height_range = Range::new(0.0, self.height as f32);

        let rect = rect_from_position(&self.position, &self.zoom);
        let real_range = Range::new(rect.start.x, rect.end.x);
        let imaginary_range = Range::new(rect.start.y, rect.end.y);

        [width_range, height_range, real_range, imaginary_range]
    }

    pub fn render(&self, pixels: &mut [u8]) {
        if self.perturbation_active {
            if let Some(orbit) = &self.reference_orbit {
                self.render_smooth_perturbation(pixels, orbit);
                return;
            }
        }

        match self.rendering {
            Rendering::Smooth => self.render_smooth(pixels),
            Rendering::Fast => self.render_fast(pixels),
        }
    }

    const SMOOTH_LUT_SIZE: usize = 4096;

    fn build_smooth_lut(&self) -> Vec<[u8; 4]> {
        (0..Self::SMOOTH_LUT_SIZE)
            .map(|i| {
                let s = i as f32 / Self::SMOOTH_LUT_SIZE as f32;
                self.color_at(s).to_rgba8()
            })
            .collect()
    }

    fn sample_lut(lut: &[[u8; 4]], s: f32) -> [u8; 4] {
        let max_index = (lut.len() - 1) as f32;
        let pos = (s * max_index).clamp(0.0, max_index);
        let idx0 = (pos as usize).min(lut.len() - 2);
        let t = pos - idx0 as f32;
        let c0 = lut[idx0];
        let c1 = lut[idx0 + 1];
        [
            (c0[0] as f32 + t * (c1[0] as f32 - c0[0] as f32)) as u8,
            (c0[1] as f32 + t * (c1[1] as f32 - c0[1] as f32)) as u8,
            (c0[2] as f32 + t * (c1[2] as f32 - c0[2] as f32)) as u8,
            0xFF,
        ]
    }

    fn build_fast_lut(&self) -> Vec<[u8; 4]> {
        (0..self.max_iterations)
            .map(|i| self.color(None, i).to_rgba8())
            .collect()
    }

    fn render_smooth(&self, pixels: &mut [u8]) {
        let [width_range, height_range, real_range, imaginary_range] = self.ranges();
        let lut = self.build_smooth_lut();

        pixels
            .par_chunks_exact_mut(4)
            .enumerate()
            .by_uniform_blocks(self.chunk_size)
            .for_each(|(index, pixel)| {
                let x = (index % self.width) as f32;
                let y = (index / self.width) as f32;

                let c = Complex32::new(
                    Range::scale(&width_range, x, &real_range),
                    Range::scale(&height_range, y, &imaginary_range),
                );

                let (z, iterations) = self.iterate(&c);
                if iterations < self.max_iterations {
                    let s = self.exponential(self.smooth(&z, iterations));
                    pixel.copy_from_slice(&Self::sample_lut(&lut, s));
                } else {
                    pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
                }
            })
    }

    fn render_fast(&self, pixels: &mut [u8]) {
        let lut = self.build_fast_lut();
        let rows = self.height / rayon::current_num_threads();
        let chunk_size = self.width * rows * 4;
        pixels
            .par_chunks_exact_mut(chunk_size)
            .enumerate()
            .for_each(|(index, pixels)| {
                let start = index * rows;
                let mut boundary_scanner = BoundaryScanner::new(self, start, start + rows);
                let data = boundary_scanner.run();
                pixels.chunks_exact_mut(4).enumerate().for_each(|(index, pixel)| {
                    let iterations = data[index] as usize;
                    if iterations < self.max_iterations {
                        pixel.copy_from_slice(&lut[iterations]);
                    } else {
                        pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
                    }
                });
            });
    }

    #[inline]
    fn is_interior(c: &Complex32) -> bool {
        let im2 = c.im * c.im;
        let mut q = c.re - 0.25;
        q *= q;
        q += im2;
        let p2 = c.re + 1.0;
        q * (q + (c.re - 0.25)) < 0.25 * im2 || p2 * p2 + im2 < 0.0625
    }

    pub(crate) fn iterate(&self, c: &Complex32) -> (Complex32, usize) {
        if Self::is_interior(c) {
            (Complex32::ZERO, self.max_iterations)
        } else {
            unsafe { self.iterate_inner(c) }
        }
    }

    #[cfg(all(not(target_arch = "aarch64"), not(target_family = "wasm")))]
    pub(crate) fn iterate_inner(&self, c: &Complex32) -> (Complex32, usize) {
        use num::traits::MulAddAssign;
        let mut z: Complex32 = Complex32::ZERO;
        let mut iterations = 0;
        let mut old: Complex32 = Complex32::ZERO;
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

    #[cfg(all(target_family = "wasm", target_feature = "simd128"))]
    pub(crate) unsafe fn iterate_inner(&self, c: &Complex32) -> (Complex32, usize) {
        use core::arch::wasm32::*;
        {
            // Pack complex into lower 2 lanes of v128: [re, im, 0, 0]
            let c = f32x4(c.re, c.im, 0.0, 0.0);
            let mut z = f32x4_splat(0.0);
            let mut iterations = 0;
            let mut old = f32x4_splat(0.0);
            let mut period = 0;
            loop {
                // z*z: (re*re - im*im, 2*re*im)
                let re = f32x4_extract_lane::<0>(z);
                let im = f32x4_extract_lane::<1>(z);
                let re2 = re * re;
                let im2 = im * im;
                z = f32x4(
                    re2 - im2 + f32x4_extract_lane::<0>(c),
                    2.0 * re * im + f32x4_extract_lane::<1>(c),
                    0.0,
                    0.0,
                );

                if f32x4_extract_lane::<0>(z) == f32x4_extract_lane::<0>(old)
                    && f32x4_extract_lane::<1>(z) == f32x4_extract_lane::<1>(old)
                {
                    let result = Complex32::new(f32x4_extract_lane::<0>(z), f32x4_extract_lane::<1>(z));
                    break (result, self.max_iterations);
                }

                iterations += 1;
                period += 1;
                if period > self.period_length {
                    period = 0;
                    old = z;
                }

                if re2 + im2 >= self.bailout || iterations >= self.max_iterations {
                    let result = Complex32::new(f32x4_extract_lane::<0>(z), f32x4_extract_lane::<1>(z));
                    break (result, iterations);
                }
            }
        }
    }

    #[cfg(all(target_arch = "aarch64", target_feature = "fcma"))]
    pub(crate) unsafe fn iterate_inner(&self, c: &Complex32) -> (Complex32, usize) {
        use core::arch::aarch64::*;
        use core::mem::transmute;
        unsafe {
            let c: float32x2_t = transmute(*c);
            let mut z = vmov_n_f32(0.0);
            let mut iterations = 0;
            let mut old = vmov_n_f32(0.0);
            let mut period = 0;
            loop {
                z = vcmla_rot90_f32(vcmla_f32(c, z, z), z, z);

                if u64::MAX == transmute::<uint32x2_t, u64>(vceq_f32(z, old)) {
                    break (transmute::<float32x2_t, Complex32>(z), self.max_iterations);
                }

                iterations += 1;
                period += 1;
                if period > self.period_length {
                    period = 0;
                    old = z;
                }

                if vaddv_f32(vmul_f32(z, z)) >= self.bailout || iterations >= self.max_iterations {
                    break (transmute::<float32x2_t, Complex32>(z), iterations);
                }
            }
        }
    }

    pub fn zoom(&mut self, x: f32, y: f32, zoom_factor: f32) {
        if let Some(hp) = &mut self.high_precision_position {
            // Compute cursor offset from center in complex plane using f64
            let cursor_frac_x = x as f64 / self.width as f64;
            let cursor_frac_y = y as f64 / self.height as f64;
            let offset_re = (cursor_frac_x * 2.0 - 1.0) * self.zoom.x;
            let offset_im = (cursor_frac_y * 2.0 - 1.0) * self.zoom.y;

            let precision = hp.x.precision();
            let offset_re_hp = dashu::float::FBig::try_from(offset_re)
                .unwrap()
                .with_precision(precision)
                .value();
            let offset_im_hp = dashu::float::FBig::try_from(offset_im)
                .unwrap()
                .with_precision(precision)
                .value();

            // Move center to cursor position
            hp.x += offset_re_hp;
            hp.y += offset_im_hp;
        }

        let width_range = Range::new(0.0, self.width as f32);
        let height_range = Range::new(0.0, self.height as f32);
        let selection = rect_from_position(&self.position, &self.zoom);
        let real_range = Range::new(selection.start.x, selection.end.x);
        let imaginary_range = Range::new(selection.start.y, selection.end.y);
        self.position = Vector {
            x: Range::scale(&width_range, x, &real_range),
            y: Range::scale(&height_range, y, &imaginary_range),
        };
        self.zoom.x *= zoom_factor as f64;
        self.zoom.y *= zoom_factor as f64;
    }

    pub fn pan(&mut self, screen_dx: f64, screen_dy: f64) {
        let width_f64 = 2.0 * self.zoom.x;
        let height_f64 = 2.0 * self.zoom.y;
        let dx = (screen_dx * width_f64 / 1000.0) as f32;
        let dy = (screen_dy * height_f64 / 1000.0) as f32;
        self.position.x -= dx;
        self.position.y -= dy;

        if let Some(hp) = &mut self.high_precision_position {
            let precision = hp.x.precision();
            let dx_hp =
                dashu::float::FBig::try_from(screen_dx * width_f64 / 1000.0)
                    .unwrap()
                    .with_precision(precision)
                    .value();
            let dy_hp =
                dashu::float::FBig::try_from(screen_dy * height_f64 / 1000.0)
                    .unwrap()
                    .with_precision(precision)
                    .value();
            hp.x -= dx_hp;
            hp.y -= dy_hp;
            // Sync f32 position from HP — at deep zoom the f32 update above
            // has no effect because dx/dy underflow relative to position.
            self.position.x = hp.x.to_f64().value() as f32;
            self.position.y = hp.y.to_f64().value() as f32;
            self.perturbation_dirty = true;
        }
    }

    fn smooth(&self, z: &Complex32, iterations: usize) -> f32 {
        if iterations < self.max_iterations {
            let zn = f32::ln(z.norm_sqr()) / 2.0;
            let nu = f32::ln(zn / std::f32::consts::LN_2) / std::f32::consts::LN_2;
            return (iterations + 1) as f32 - nu;
        }
        iterations as f32
    }

    fn exponential(&self, iterations: f32) -> f32 {
        f32::powf(iterations / self.max_iterations as f32, self.exponent)
    }

    pub fn color_at(&self, s: f32) -> Color {
        match self.coloring {
            Coloring::Palette => {
                let (_, palette) = &self.palettes[self.selected_palette];
                palette.at(f32::powf(s, 1.0 / 3.0))
            }
            Coloring::LCH => {
                let s = f32::powf(s, 1.0 / 3.0);
                let v = 1.0 - f32::powf(f32::cos(std::f32::consts::PI * s), 2.0);
                Color::from_lcha(
                    75.0 - (75.0 * v),
                    28.0 + (75.0 - (75.0 * v)),
                    360.0 * s % 360.0,
                    1.0,
                )
            }
        }
    }

    fn color(&self, z: Option<&Complex32>, iterations: usize) -> Color {
        let smooth = if let Some(z) = z {
            self.smooth(z, iterations)
        } else {
            iterations as f32
        };
        self.color_at(self.exponential(smooth))
    }
}

impl Mandelbrot {
    /// Update HP position for zoom toward cursor. Call before modifying zoom.
    /// `cursor_frac_x/y` are cursor position as fraction of window size (0..1).
    pub fn zoom_hp(&mut self, cursor_frac_x: f64, cursor_frac_y: f64, zoom_factor: f64) {
        if let Some(hp) = &mut self.high_precision_position {
            let offset_re =
                self.zoom.x * (2.0 * cursor_frac_x - 1.0) * (1.0 - zoom_factor);
            let offset_im =
                self.zoom.y * (2.0 * cursor_frac_y - 1.0) * (1.0 - zoom_factor);
            let precision = hp.x.precision();
            hp.x += dashu::float::FBig::try_from(offset_re)
                .unwrap()
                .with_precision(precision)
                .value();
            hp.y += dashu::float::FBig::try_from(offset_im)
                .unwrap()
                .with_precision(precision)
                .value();
        }
        self.perturbation_dirty = true;
    }

    pub fn update_perturbation_state(&mut self) {
        if needs_perturbation(self.zoom.x, self.width) {
            // Skip recomputation if nothing changed
            if self.perturbation_active && !self.perturbation_dirty {
                return;
            }

            // Promote f32 position to HP on first activation
            if self.high_precision_position.is_none() {
                let precision = required_precision_bits(self.zoom.x);
                self.high_precision_position = Some(HighPrecisionPosition::from_f32(
                    self.position.x,
                    self.position.y,
                    precision,
                ));
            }

            // Increase precision if needed
            if let Some(hp) = &mut self.high_precision_position {
                let needed = required_precision_bits(self.zoom.x);
                if hp.x.precision() < needed {
                    hp.x = hp.x.clone().with_precision(needed).value();
                    hp.y = hp.y.clone().with_precision(needed).value();
                }
            }

            let hp = self.high_precision_position.as_ref().unwrap();
            let center_re = hp.x.to_f64().value();
            let center_im = hp.y.to_f64().value();
            let precision = hp.x.precision();

            // Try to reuse existing reference point — just update the offset
            let need_new_ref = if let Some(ref rp) = self.reference_point {
                // Compute offset in HIGH PRECISION first, then convert to f64.
                // Converting rp and hp to f64 separately and subtracting loses precision
                // when both are near the same value (e.g., -0.67) and the difference is
                // tiny (e.g., 1e-33) — f64 subtraction gives 0 due to catastrophic cancellation.
                let off_re = (&rp.re - &hp.x).to_f64().value();
                let off_im = (&rp.im - &hp.y).to_f64().value();

                // Check if reference point is still within the viewport
                let in_viewport = off_re.abs() < 2.0 * self.zoom.x
                    && off_im.abs() < 2.0 * self.zoom.y;

                // Check if orbit was computed with at least current max_iterations.
                // An orbit that escapes early is fine — pixels past that point use fallback.
                // Recomputing won't produce a longer orbit at the same reference point.
                let orbit_ok = self
                    .reference_orbit
                    .as_ref()
                    .is_some_and(|o| o.computed_max_iterations >= self.max_iterations);

                if in_viewport && orbit_ok {
                    // Reuse — just update offset (no orbit recomputation!)
                    self.reference_offset = (off_re, off_im);
                    // Recompute SA with current zoom's max_delta.
                    // On zoom-in: SA converges better → more skip iterations.
                    // On zoom-out: SA tolerance fails earlier → fewer skip iterations.
                    // Always update to match the current viewport scale.
                    // max_delta must cover the worst-case |dc| including reference offset.
                    let max_delta = (self.zoom.x + self.reference_offset.0.abs())
                        .hypot(self.zoom.y + self.reference_offset.1.abs());
                    if let Some(orbit) = &self.reference_orbit {
                        let new_sa = recompute_sa(orbit, max_delta, self.sa_order);
                        let current_sa = orbit.sa_coefficients.as_ref();
                        let needs_update = match (&new_sa, current_sa) {
                            (Some(new), Some(old)) => {
                                new.skip_iterations != old.skip_iterations
                                    || (new.scale - old.scale).abs() > f64::EPSILON
                            }
                            (Some(_), None) | (None, Some(_)) => true,
                            (None, None) => false,
                        };
                        if needs_update {
                            let mut updated = (**orbit).clone();
                            updated.sa_coefficients = new_sa;
                            self.reference_orbit = Some(Arc::new(updated));
                        }
                    }
                    false
                } else {
                    true
                }
            } else {
                true
            };

            if need_new_ref {
                self.find_and_compute_reference(center_re, center_im, precision);
            }

            self.perturbation_active = true;
            self.perturbation_dirty = false;
        } else {
            self.perturbation_active = false;
            // Keep HP position to avoid precision loss if we zoom back in
        }
    }

    fn find_and_compute_reference(
        &mut self,
        center_re: f64,
        center_im: f64,
        precision: usize,
    ) {
        // Test the center first
        let center_length = test_orbit_f64(
            center_re,
            center_im,
            self.max_iterations,
            self.bailout as f64,
        );

        if center_length >= self.max_iterations {
            // Center is interior, use directly
            let hp = self.high_precision_position.as_ref().unwrap();
            let ref_center = HighPrecisionComplex::new(hp.x.clone(), hp.y.clone());
            let max_delta = self.zoom.x.hypot(self.zoom.y);
            let orbit = compute_reference_orbit_with_sa(&ref_center, self.max_iterations, self.bailout, max_delta, self.sa_order);
            self.reference_orbit = Some(Arc::new(orbit));
            self.reference_offset = (0.0, 0.0);
            self.reference_point = Some(ref_center);
            return;
        }

        // Search nearby for a longer-lived reference
        let mut best_offset = (0.0, 0.0);
        let mut best_length = center_length;

        let steps = [-0.4, -0.2, -0.1, 0.0, 0.1, 0.2, 0.4];
        'search: for &fx in &steps {
            for &fy in &steps {
                if fx == 0.0 && fy == 0.0 {
                    continue;
                }
                let off_re = fx * 2.0 * self.zoom.x;
                let off_im = fy * 2.0 * self.zoom.y;
                let length = test_orbit_f64(
                    center_re + off_re,
                    center_im + off_im,
                    self.max_iterations,
                    self.bailout as f64,
                );
                if length > best_length {
                    best_length = length;
                    best_offset = (off_re, off_im);
                    if length >= self.max_iterations {
                        break 'search;
                    }
                }
            }
        }

        // Build reference point at the best location
        let off_re_hp = dashu::float::FBig::try_from(best_offset.0)
            .unwrap()
            .with_precision(precision)
            .value();
        let off_im_hp = dashu::float::FBig::try_from(best_offset.1)
            .unwrap()
            .with_precision(precision)
            .value();
        let hp = self.high_precision_position.as_ref().unwrap();
        let ref_center = HighPrecisionComplex::new(
            hp.x.clone() + off_re_hp,
            hp.y.clone() + off_im_hp,
        );
        // max_delta must cover worst-case |dc| including reference offset
        let max_delta = (self.zoom.x + best_offset.0.abs())
            .hypot(self.zoom.y + best_offset.1.abs());
        let orbit = compute_reference_orbit_with_sa(
            &ref_center,
            self.max_iterations,
            self.bailout,
            max_delta,
            self.sa_order,
        );
        self.reference_orbit = Some(Arc::new(orbit));
        self.reference_offset = (best_offset.0, best_offset.1);
        self.reference_point = Some(ref_center);
    }

    fn render_smooth_perturbation(&self, pixels: &mut [u8], orbit: &ReferenceOrbit) {
        let lut = self.build_smooth_lut();
        let bailout = self.bailout as f64;
        let zoom_x = self.zoom.x;
        let zoom_y = self.zoom.y;
        let ref_off_re = self.reference_offset.0;
        let ref_off_im = self.reference_offset.1;
        let width = self.width;
        let height = self.height;
        let max_iterations = self.max_iterations;
        let exponent = self.exponent;

        pixels
            .par_chunks_exact_mut(4)
            .enumerate()
            .by_uniform_blocks(self.chunk_size)
            .for_each(|(index, pixel)| {
                let px = (index % width) as f64;
                let py = (index / width) as f64;

                let dc_re = (px / width as f64 * 2.0 - 1.0) * zoom_x - ref_off_re;
                let dc_im = (py / height as f64 * 2.0 - 1.0) * zoom_y - ref_off_im;

                let (norm_sq, iterations) =
                    iterate_perturbation(orbit, dc_re, dc_im, max_iterations, bailout);

                if iterations < max_iterations {
                    let norm_sq_f32 = norm_sq as f32;
                    let zn = norm_sq_f32.ln() / 2.0;
                    let nu = f32::ln(zn / std::f32::consts::LN_2) / std::f32::consts::LN_2;
                    let smooth = (iterations + 1) as f32 - nu;
                    let s = f32::powf(smooth / max_iterations as f32, exponent);
                    pixel.copy_from_slice(&Self::sample_lut(&lut, s));
                } else {
                    pixel.copy_from_slice(&[0, 0, 0, 0xFF]);
                }
            });
    }
}

impl Default for Mandelbrot {
    fn default() -> Self {
        let palettes = [
            ("Rust", vec!["#3e0000", "#6b1d09", "#9a542e", "#bf935c", "#d0c8a8"]),
            ("Cold Teal", vec!["#1B3A4B", "#3D6B7E", "#71C9CE", "#A6E3E9", "#E3FDFD"]),
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
            width: 1280,
            height: 720,
            position: Vector { x: -0.5, y: 0.0 },
            zoom: Vector { x: 2.0, y: 1.125 },
            rendering: Rendering::Fast,
            bailout: f32::powf(2.0, 16.0),
            max_iterations: 1000,
            chunk_size: usize::pow(2, 8),
            period_length: 20,
            coloring: Coloring::LCH,
            exponent: 1.0,
            palettes,
            selected_palette: 0,
            high_precision_position: None,
            reference_orbit: None,
            perturbation_active: false,
            reference_offset: (0.0, 0.0),
            reference_point: None,
            perturbation_dirty: true,
            sa_order: 6,
        }
    }
}
