use clap::{AppSettings, Clap};
use color_eyre::eyre;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clap)]
#[clap(
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_setting = AppSettings::ColorAuto,
    global_setting = AppSettings::ColoredHelp,
    version = env!("CARGO_PKG_VERSION"),
)]
struct Options {
    /// Audio sample file path
    #[clap()]
    file: PathBuf,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let options = Options::parse();

    let file = File::open(options.file)?;
    let reader = BufReader::new(file);

    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let source = Decoder::new(reader)?;
    sink.append(source);

    sink.sleep_until_end();

    Ok(())
}
