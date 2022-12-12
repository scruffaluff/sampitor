//! Structs for reusable knobs.

use crate::view::filter::base::Knob;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FloatKnob {
    pub maximum: f32,
    pub minimum: f32,
    pub step: f32,
    text: String,
    pub value: f32,
}

impl Default for FloatKnob {
    fn default() -> Self {
        Self {
            maximum: 1.0,
            minimum: 0.0,
            step: 0.1,
            text: 1.0.to_string(),
            value: 1.0,
        }
    }
}

impl Knob for FloatKnob {
    fn decrement(&mut self) {
        self.value = (self.value - self.step).max(self.minimum);
        self.text = self.value.to_string();
    }

    fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.maximum);
        self.text = self.value.to_string();
    }

    fn text(&self) -> &str {
        &self.text
    }
}
