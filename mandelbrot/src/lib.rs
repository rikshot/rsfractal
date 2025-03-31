#![cfg_attr(all(target_arch = "aarch64", target_feature = "fcma"), feature(stdarch_neon_fcma))]

pub mod boundary_scanner;
pub mod mandelbrot;
pub mod range;
pub mod rectangle;
pub mod vector;
