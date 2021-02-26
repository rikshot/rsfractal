use serde::{Deserialize, Serialize};

use num_traits::Num;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector<T: Num> {
    pub x: T,
    pub y: T,
}

impl<T: Num> Vector<T> {
    pub fn new(x: T, y: T) -> Vector<T> {
        Vector { x, y }
    }
}
