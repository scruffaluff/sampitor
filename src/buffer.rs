use rodio::buffer;

/// A wrapper around Rodio's SamplesBuffer to allow for repeated playback and additional processing.
pub struct SamplesBuffer {
    pub data: Vec<f32>,
    pub channels: u16,
    pub sample_rate: u32,
}

impl SamplesBuffer {
    /// Create a new SamplesBuffer from a signal an audio metadata.
    pub fn new(channels: u16, sample_rate: u32, data: Vec<f32>) -> Self {
        SamplesBuffer {
            data,
            channels,
            sample_rate,
        }
    }
}

impl From<&SamplesBuffer> for buffer::SamplesBuffer<f32> {
    /// Copy into a Rodio source for playback.
    fn from(value: &SamplesBuffer) -> Self {
        buffer::SamplesBuffer::new(value.channels, value.sample_rate, value.data.clone())
    }
}
