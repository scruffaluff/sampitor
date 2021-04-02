use crate::buffer::SamplesBuffer;
use color_eyre::eyre;
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// Get path file name or descriptive error.
pub fn name(path: &Path) -> eyre::Result<&str> {
    path.file_name()
        .ok_or_else(|| eyre::eyre!("File path {:?} does not have a final component", path))?
        .to_str()
        .ok_or_else(|| eyre::eyre!("File name {:?} is not valid Unicode", path))
}

/// Read audio metdata and samples.
pub fn read_samples(path: &Path) -> eyre::Result<SamplesBuffer> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let source = Decoder::new(reader)?;

    let channels = source.channels();
    let sample_rate = source.sample_rate();
    let samples: Vec<f32> = source.convert_samples().buffered().collect();
    Ok(SamplesBuffer::new(channels, sample_rate, samples))
}
