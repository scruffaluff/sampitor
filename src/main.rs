use clap::{AppSettings, Clap};
use color_eyre::eyre;
use std::path::{Path, PathBuf};
use std::str;

#[derive(Clap)]
#[clap(
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_setting = AppSettings::ColorAuto,
    global_setting = AppSettings::ColoredHelp,
    version = env!("CARGO_PKG_VERSION"),
)]
struct Options {
    /// Increase the verbosity of messages
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let options = Options::parse();

    Ok(())
}
