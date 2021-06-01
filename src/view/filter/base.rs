//! Fundemental traits for digital signal processing user interface components.

use crate::dsp::buffer::Samples;

pub trait Knob {
    fn decrement(&mut self);
    fn increment(&mut self);
    fn text(&self) -> &str;
}

pub trait Filter {
    fn knobs(&mut self) -> Vec<(&str, &mut dyn Knob)>;
    fn process(&mut self, samples: &mut Samples);
}
