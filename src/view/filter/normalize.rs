//! User interface for changing amplitude levels of a signal.

use crate::dsp::buffer::Samples;
use crate::dsp::filters::normalize;
use crate::view::filter::base::{Filter, Knob};

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
        normalize(self.amplitude.value, samples);
    }
}
