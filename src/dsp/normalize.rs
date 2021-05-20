use crate::dsp::base::{Filter, Knob};
use crate::dsp::buffer::Samples;
use std::cmp::Ordering;

#[derive(Debug)]
struct Amplitude {
    value: f32,
}

impl Default for Amplitude {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Knob for Amplitude {
    fn decrement(&mut self) {
        self.value = (self.value - 0.1).max(0.0);
    }

    fn increment(&mut self) {
        self.value = (self.value + 0.1).min(1.0);
    }

    fn get_max(&self) -> u64 {
        100_u64
    }

    // Variable self.value should always be between 0 and 1, so sign loss should not be possible.
    #[allow(clippy::cast_sign_loss)]
    fn get_value(&self) -> u64 {
        (self.value * (self.get_max() as f32)) as u64
    }
}

#[derive(Debug, Default)]
pub struct Normalize {
    amplitude: Amplitude,
}

impl Filter for Normalize {
    fn knobs(&mut self) -> Vec<(&str, &mut dyn Knob)> {
        vec![("Amplitude", &mut self.amplitude)]
    }

    fn process(&mut self, samples: &mut Samples) {
        let maximum = samples
            .data
            .iter()
            .map(|x| f32::abs(*x))
            .max_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal));

        if let Some(maximum) = maximum {
            let scale = maximum / self.amplitude.value;
            samples.data.iter_mut().for_each(|x| *x /= scale);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_default() {
        let mut filter = Normalize::default();
        let mut actual = Samples::new(2, 20, vec![-0.5, -0.25, 0.25, 0.0]);

        filter.process(&mut actual);

        let expected = Samples::new(2, 20, vec![-1.0, -0.5, 0.5, 0.0]);
        assert_eq!(actual.data, expected.data);
    }
}
