use num_traits::float::Float;

pub struct Range<T: Float> {
    min: T,
    max: T,
}

impl<T: Float> Range<T> {
    pub fn new(min: T, max: T) -> Range<T> {
        Range { min, max }
    }

    pub fn scale(input: &Range<T>, value: T, output: &Range<T>) -> T {
        let input_size = T::abs(input.max - input.min);
        let output_size = T::abs(output.max - output.min);
        (input.max * output.min - input.min * output.max + value * output_size) / input_size
    }
}
