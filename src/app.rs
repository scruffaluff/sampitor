use crate::chart::SignalChart;
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use rodio::buffer::SamplesBuffer;
use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Stdout;
use std::path::{Path, PathBuf};
use tui::backend::CrosstermBackend;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

type CrossTerm = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
    buffer: Vec<f32>,
    channels: u16,
    name: String,
    sample_rate: u32,
    signal_chart: SignalChart<'static>,
    sink: Sink,
    // If stream is dropped then sound will not reach the speakers.
    _stream: OutputStream,
    terminal: CrossTerm,
}

impl App {
    pub fn try_new(path: PathBuf) -> eyre::Result<Self> {
        let file_name = path
            .file_name()
            .ok_or(eyre::eyre!(
                "File path {:?} does not have a final component",
                path
            ))?
            .to_str()
            .ok_or(eyre::eyre!("File name {:?} is not valid Unicode", path))?;

        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        let (channels, sample_rate, buffer) = read_buffer(&path)?;
        let mut signal_chart = SignalChart::new(channels as usize, buffer.len());
        signal_chart.update(&buffer);

        Ok(App {
            buffer: buffer,
            channels: channels,
            name: format!("Sampitor: {}", file_name),
            sample_rate: sample_rate,
            signal_chart: signal_chart,
            sink: sink,
            _stream: stream,
            terminal: take_terminal()?,
        })
    }

    fn draw(&mut self) -> io::Result<()> {
        let chart = self.signal_chart.render();

        let name: &str = &self.name;
        self.terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Percentage(20), Percentage(80)].as_ref())
                .split(size);

            let block = Block::default().title(name).borders(Borders::ALL);
            frame.render_widget(block, chunks[0]);
            frame.render_widget(chart, chunks[1]);
        })?;

        Ok(())
    }

    fn play(&self) -> eyre::Result<()> {
        let buffer = SamplesBuffer::new(self.channels, self.sample_rate, self.buffer.clone());
        Ok(self.sink.append(buffer))
    }

    pub fn run(&mut self) -> eyre::Result<()> {
        loop {
            self.draw()?;

            match event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        self.play()?;
                    }
                    _ => (),
                },
                _ => (),
            }

            self.update();
        }

        leave_terminal(&mut self.terminal)?;

        Ok(())
    }

    fn update(&mut self) {}
}

fn leave_terminal(terminal: &mut CrossTerm) -> eyre::Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    Ok(())
}

fn read_buffer(path: &Path) -> eyre::Result<(u16, u32, Vec<f32>)> {
    let file = File::open(&path)?;
    let reader = BufReader::new(file);
    let source = Decoder::new(reader)?;

    let channels = source.channels();
    let sample_rate = source.sample_rate();
    let samples: Vec<f32> = source.convert_samples().buffered().collect();
    Ok((channels, sample_rate, samples))
}

fn take_terminal() -> eyre::Result<CrossTerm> {
    terminal::enable_raw_mode()?;
    let mut screen = io::stdout();
    execute!(screen, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(screen);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
