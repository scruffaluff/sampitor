//! Fundemental traits for digital signal processing user interface components.

use crate::dsp::buffer::Samples;

pub trait Knob {
    fn decrement(&mut self);
    fn increment(&mut self);
    fn text(&self) -> &str;
}

/// Blanket ergonomic mplementation for &mut K.
impl<K: Knob> Knob for &mut K {
    fn decrement(&mut self) {
        K::decrement(self);
    }

    fn increment(&mut self) {
        K::increment(self);
    }

    fn text(&self) -> &str {
        K::text(self)
    }
}

pub trait Filter {
    fn knobs(&mut self) -> Vec<(&str, &mut dyn Knob)>;
    fn process(&mut self, samples: &mut Samples);
}

/// Blanket ergonomic mplementation for &mut F.
impl<F: Filter> Filter for &mut F {
    fn knobs(&mut self) -> Vec<(&str, &mut dyn Knob)> {
        F::knobs(self)
    }

    fn process(&mut self, samples: &mut Samples) {
        F::process(self, samples);
    }
}
