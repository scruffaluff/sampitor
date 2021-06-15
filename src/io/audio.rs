//! Audio readers and writers.

use crate::dsp::Samples;
use color_eyre::eyre;
use hound::{SampleFormat, WavSpec, WavWriter};
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Read audio metadata and samples from a file.
///
/// # Errors
///
/// Will return `Err` if `path` cannot be opened or contains invalid audio data.
pub fn read_samples(path: &Path) -> eyre::Result<Samples> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let source = Decoder::new(reader)?;

    let channels = source.channels();
    let sample_rate = source.sample_rate();
    let samples: Vec<f32> = source.convert_samples().buffered().collect();
    Ok(Samples::new(channels, sample_rate, samples))
}

/// Write audio metdata and samples to a file.
///
/// # Errors
///
/// Will return `Err` if `path` is unwritable.
pub fn write_samples(path: &Path, samples: &Samples) -> eyre::Result<()> {
    let spec = WavSpec {
        channels: samples.channels,
        sample_rate: samples.sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec)?;

    for sample in &samples.data {
        writer.write_sample(*sample)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    use approx::assert_abs_diff_eq;

    #[test]
    fn write_and_read() {
        let expected = Samples::new(2, 32, vec![0.0f32, -0.25f32, 0.25f32, 1.0f32]);
        let path = util::test::temp_wave_file(&expected).unwrap();

        let actual = read_samples(&path).unwrap();
        assert_abs_diff_eq!(actual, expected, epsilon = 0.0001);
    }
}
