use clap::{AppSettings, Clap};
use color_eyre::eyre;
use sampitor;
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

    sampitor::play_file(&options.file)?;

    Ok(())
}
