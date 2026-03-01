use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Vector<T = f32> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
