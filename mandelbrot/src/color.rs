#[cfg(not(target_arch = "wasm32"))]
use regex::Regex;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn from_hex(hex: &str) -> Option<Color> {
        lazy_static! {
            static ref HEX_RE: Regex = Regex::new(r"^#?([a-fA-F\d]{2})([a-fA-F\d]{2})([a-fA-F\d]{2})$").unwrap();
        }
        let raw_colors = HEX_RE.captures(hex)?;
        let red = u64::from_str_radix(&raw_colors[1], 16).ok()?;
        let green = u64::from_str_radix(&raw_colors[2], 16).ok()?;
        let blue = u64::from_str_radix(&raw_colors[3], 16).ok()?;
        Some(Color {
            r: red as u8,
            g: green as u8,
            b: blue as u8,
        })
    }

    #[cfg(target_arch = "wasm32")]
    pub fn from_hex(hex: &str) -> Option<Color> {
        let hex_re: js_sys::RegExp = js_sys::RegExp::new(r"^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$", "i");
        let hex_string = js_sys::JsString::from(hex);
        let raw_colors = wasm_bindgen::JsCast::unchecked_into::<js_sys::Array>(hex_string.match_(&hex_re)?);
        let red = u64::from_str_radix(
            &String::from(wasm_bindgen::JsCast::unchecked_into::<js_sys::JsString>(
                raw_colors.get(1),
            )),
            16,
        )
        .ok()?;
        let green = u64::from_str_radix(
            &String::from(wasm_bindgen::JsCast::unchecked_into::<js_sys::JsString>(
                raw_colors.get(2),
            )),
            16,
        )
        .ok()?;
        let blue = u64::from_str_radix(
            &String::from(wasm_bindgen::JsCast::unchecked_into::<js_sys::JsString>(
                raw_colors.get(3),
            )),
            16,
        )
        .ok()?;
        Some(Color {
            r: red as u8,
            g: green as u8,
            b: blue as u8,
        })
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
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

    #[test]
    fn hex() {
        assert_eq!(Color::from_hex("#FF0000").unwrap(), Color { r: 255, g: 0, b: 0 });
        assert_eq!(Color::from_hex("00ff00").unwrap(), Color { r: 0, g: 255, b: 0 });
    }

    #[test]
    #[should_panic]
    fn hex_panic() {
        Color::from_hex("invalid").unwrap();
    }
}
