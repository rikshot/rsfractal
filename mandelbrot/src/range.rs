#[derive(Clone, Copy)]
pub struct Range {
    min: f64,
    max: f64,
}

impl Range {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn scale(input: &Self, value: f64, output: &Self) -> f64 {
        let input_size = f64::abs(input.max - input.min);
        let output_size = f64::abs(output.max - output.min);
        (input.max * output.min - input.min * output.max + value * output_size) / input_size
    }
}
