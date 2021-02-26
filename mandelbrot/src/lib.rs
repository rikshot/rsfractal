#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate lazy_static;

pub mod color;
pub mod mandelbrot;
pub mod range;
pub mod rectangle;
pub mod vector;
