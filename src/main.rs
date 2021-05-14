use clap::{AppSettings, Clap};
use color_eyre::eyre;
use rodio::{OutputStream, Sink};
use sampitor::dsp::Samples;
use sampitor::io::{self, path};
use sampitor::view::{Chart, File, View};
use sampitor::App;
use std::convert::TryInto;
use std::env;
use std::io::Stdout;
use std::path::PathBuf;
use tui::backend::CrosstermBackend;

#[derive(Clap)]
#[clap(
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_setting = AppSettings::ColorAuto,
    global_setting = AppSettings::ColoredHelp,
    version = env!("CARGO_PKG_VERSION"),
)]
struct Options {
    /// Audio file search directory
    #[clap(short, long)]
    dir: Option<PathBuf>,
    /// Audio sample file path
    #[clap(short, long)]
    file: Option<PathBuf>,
}

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let options = Options::parse();

    let mut terminal = io::terminal::take()?;
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let samples = match options.file {
        Some(file_path) => path::read_samples(&file_path)?,
        None => Samples::default(),
    };
    let channels: usize = samples.channels.try_into()?;

    let mut chart = Chart::new(String::new(), channels, samples.data.len() / channels);
    let mut file = match options.dir {
        Some(directory) => File::try_new(directory)?,
        None => File::try_new(env::current_dir()?)?,
    };

    let mut views: Vec<(&str, &mut dyn View<CrosstermBackend<Stdout>>)> = Vec::new();
    views.push(("Chart", &mut chart));
    views.push(("File", &mut file));

    let mut app = App::new(&mut views, samples);
    app.run(&mut terminal, &sink)?;

    io::terminal::leave(&mut terminal)?;

    Ok(())
}
