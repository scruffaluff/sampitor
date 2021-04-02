use color_eyre::eyre;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn play_file(path: &Path) -> eyre::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let source = Decoder::new(reader)?;
    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}
