extern crate serde;

use serde::Deserialize;

extern crate num_traits;

use num_traits::Num;

use super::vector::Vector;

#[derive(Debug, Clone, Deserialize)]
pub struct Rectangle<T: Num> {
    pub start: Vector<T>,
    pub end: Vector<T>,
}

impl<T: Num + Copy> Rectangle<T> {
    pub fn new(start: Vector<T>, end: Vector<T>) -> Rectangle<T> {
        Rectangle { start, end }
    }

    pub fn width(&self) -> T {
        self.end.x - self.start.x
    }

    pub fn height(&self) -> T {
        self.end.y - self.start.y
    }
}
