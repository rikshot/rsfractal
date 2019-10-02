use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn lerp(color1: &Color, color2: &Color, value: f64) -> Color {
        Color {
            r: (color1.r as f64 + (color2.r as f64 - color1.r as f64) * value) as u8,
            g: (color1.g as f64 + (color2.g as f64 - color1.g as f64) * value) as u8,
            b: (color1.b as f64 + (color2.b as f64 - color1.b as f64) * value) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp() {
        let color1 = Color::new(0, 0, 0);
        let color2 = Color::new(255, 255, 255);
        let color = Color::lerp(&color1, &color2, 0.5);
        assert_eq!(color.r, 127);
        assert_eq!(color.g, 127);
        assert_eq!(color.b, 127);
    }
}
