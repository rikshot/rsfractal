#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Range {
    min: f32,
    max: f32,
}

impl Range {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn scale(input: &Self, value: f32, output: &Self) -> f32 {
        let input_size = f32::abs(input.max - input.min);
        let output_size = f32::abs(output.max - output.min);
        (input.max * output.min - input.min * output.max + value * output_size) / input_size
    }
}
