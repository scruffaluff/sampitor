use clap::{AppSettings, Clap};
use color_eyre::eyre;
use rodio::{OutputStream, Sink};
use sampitor::io;
use sampitor::App;
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

    let mut terminal = io::terminal::take()?;
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let mut app = App::try_new(&options.file)?;
    app.run(&mut terminal, &sink)?;

    io::terminal::leave(&mut terminal)?;

    Ok(())
}
