use super::vector::Vector;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub start: Vector,
    pub end: Vector,
}

impl Rectangle {
    pub fn new(start: Vector, end: Vector) -> Self {
        Rectangle { start, end }
    }

    pub fn width(&self) -> f64 {
        self.end.x - self.start.x
    }

    pub fn height(&self) -> f64 {
        self.end.y - self.start.y
    }
}
