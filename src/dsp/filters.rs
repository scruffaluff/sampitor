//! Algorithms for filtering signals.

use crate::dsp::buffer::Samples;
use std::cmp::Ordering;

pub fn normalize(amplitude: f32, samples: &mut Samples) {
    let maximum = samples
        .data
        .iter()
        .map(|x| f32::abs(*x))
        .max_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal));

    if let Some(maximum) = maximum {
        let scale = maximum / amplitude;
        samples.data.iter_mut().for_each(|x| *x /= scale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_default() {
        let mut actual = Samples::new(2, 20, vec![-0.5, -0.25, 0.25, 0.0]);

        normalize(1.0_f32, &mut actual);

        let expected = Samples::new(2, 20, vec![-1.0, -0.5, 0.5, 0.0]);
        assert_eq!(actual.data, expected.data);
    }
}
