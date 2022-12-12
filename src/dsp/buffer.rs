//! Convenience structs for digital signal processing.

use rodio::buffer::SamplesBuffer;

/// A wrapper around Rodio's Samples to allow for repeated playback and additional processing.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Samples {
    pub data: Vec<f32>,
    pub channels: u16,
    pub sample_rate: u32,
}

impl Samples {
    /// Create a new Samples from a signal an audio metadata.
    #[must_use]
    pub fn new(channels: u16, sample_rate: u32, data: Vec<f32>) -> Self {
        Self {
            data,
            channels,
            sample_rate,
        }
    }
}

impl Default for Samples {
    fn default() -> Self {
        Self {
            data: Vec::default(),
            channels: 1,
            sample_rate: 1,
        }
    }
}

impl From<&Samples> for SamplesBuffer<f32> {
    /// Copy into a Rodio source for playback.
    fn from(value: &Samples) -> Self {
        Self::new(value.channels, value.sample_rate, value.data.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::AbsDiffEq;

    impl AbsDiffEq for Samples {
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
