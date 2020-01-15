pub mod color;
pub mod mandelbrot;
pub mod range;
pub mod rectangle;
pub mod vector;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub mod ui;
