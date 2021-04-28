use rodio::buffer;

/// A wrapper around Rodio's SamplesBuffer to allow for repeated playback and additional processing.
#[derive(Debug, PartialEq)]
pub struct SamplesBuffer {
    pub data: Vec<f32>,
    pub channels: u16,
    pub sample_rate: u32,
}

impl SamplesBuffer {
    /// Create a new SamplesBuffer from a signal an audio metadata.
    pub fn new(channels: u16, sample_rate: u32, data: Vec<f32>) -> Self {
        SamplesBuffer {
            data,
            channels,
            sample_rate,
        }
    }
}

impl From<&SamplesBuffer> for buffer::SamplesBuffer<f32> {
    /// Copy into a Rodio source for playback.
    fn from(value: &SamplesBuffer) -> Self {
        buffer::SamplesBuffer::new(value.channels, value.sample_rate, value.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::AbsDiffEq;

    impl AbsDiffEq for SamplesBuffer {
        type Epsilon = f32;
        fn default_epsilon() -> f32 {
            f32::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: f32) -> bool {
            vec![
                self.channels == other.channels,
                self.sample_rate == other.sample_rate,
                self.data.len() == self.data.len(),
                self.data
                    .iter()
                    .zip(other.data.iter())
                    .all(|(x, y)| f32::abs_diff_eq(x, y, epsilon)),
            ]
            .iter()
            .all(|bool| *bool)
        }
    }
}
