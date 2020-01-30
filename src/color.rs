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

struct HSV {
    hue: f64,
    saturation: f64,
    value: f64,
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
            &&String::from(wasm_bindgen::JsCast::unchecked_into::<js_sys::JsString>(
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

    pub fn lerp_hsv(color1: &Color, color2: &Color, value: f64) -> Color {
        let mut color1_hsv = Color::to_hsv(color1);
        let mut color2_hsv = Color::to_hsv(color2);

        let mut d = color2_hsv.hue - color1_hsv.hue;
        let mut t = value;
        if color1_hsv.hue > color2_hsv.hue {
            let h3 = color2_hsv.hue;
            color2_hsv.hue = color1_hsv.hue;
            color1_hsv.hue = h3;
            d = -d;
            t = 1.0 - t;
        }

        let hsv = HSV {
            hue: if d > 0.5 {
                (color1_hsv.hue + 1.0 + t * (color2_hsv.hue - color1_hsv.hue)) % 1.0
            } else {
                color1_hsv.hue + t * d
            },
            saturation: color1_hsv.saturation + (color2_hsv.saturation - color1_hsv.saturation) * value,
            value: color1_hsv.value + (color2_hsv.value - color1_hsv.value) * value,
        };

        Color::to_rgb(&hsv)
    }

    fn to_hsv(color: &Color) -> HSV {
        let r = color.r as f64 / 255.0;
        let g = color.g as f64 / 255.0;
        let b = color.b as f64 / 255.0;
        let max = max(&[r, g, b]);
        let min = min(&[r, g, b]);
        let mut hue = if max == min {
            0.0
        } else if max == r {
            60.0 * (0.0 + ((g - b) / (max - min)))
        } else if max == g {
            60.0 * (2.0 + ((b - r) / (max - min)))
        } else {
            60.0 * (4.0 + ((r - g) / (max - min)))
        };
        if hue < 0.0 {
            hue += 360.0;
        }
        let saturation = if max == 0.0 { 0.0 } else { (max - min) / max };
        HSV {
            hue,
            saturation,
            value: max,
        }
    }

    fn to_rgb(hsv: &HSV) -> Color {
        Color::new(
            (convert(hsv, 5.0) * 255.0) as u8,
            (convert(hsv, 3.0) * 255.0) as u8,
            (convert(hsv, 1.0) * 255.0) as u8,
        )
    }
}

fn convert(hsv: &HSV, n: f64) -> f64 {
    let k = (n + hsv.hue as f64 / 60.0) % 6.0;
    hsv.value - hsv.value * hsv.saturation * max(&[min(&[k, 4.0 - k, 1.0]), 0.0])
}

fn max(array: &[f64]) -> f64 {
    array.iter().cloned().fold(std::f64::NEG_INFINITY, |a, b| a.max(b))
}

fn min(array: &[f64]) -> f64 {
    array.iter().cloned().fold(std::f64::INFINITY, |a, b| a.min(b))
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
    fn to_hsv() {
        let color = Color::new(255, 0, 0);
        let hsv = Color::to_hsv(&color);
        assert_eq!(hsv.hue, 0.0);
        assert_eq!(hsv.saturation, 1.0);
        assert_eq!(hsv.value, 1.0);
    }

    #[test]
    fn to_rgb() {
        let hsv = HSV {
            hue: 0.0,
            saturation: 1.0,
            value: 1.0,
        };
        let color = Color::to_rgb(&hsv);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
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
