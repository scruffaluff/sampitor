//! Structs for reusable knobs.

use crate::view::filter::base::Knob;

#[derive(Debug)]
pub struct Amplitude {
    pub value: f32,
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
