//! User interface for changing amplitude levels of a signal.

use crate::dsp::buffer::Samples;
use crate::dsp::filters::normalize;
use crate::view::filter::base::{Filter, Knob};
use crate::view::filter::knobs::Amplitude;

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
