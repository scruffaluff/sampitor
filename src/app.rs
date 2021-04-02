use crate::chart::SignalChart;
use crate::file;
use crate::terminal::{self, CrossTerm};
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode};
use rodio::buffer::SamplesBuffer;
use rodio::{OutputStream, Sink};
use std::io;
use std::path::PathBuf;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};
use tui::widgets::{Block, Borders};

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
        let file_name = file::name(&path)?;

        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        let (channels, sample_rate, buffer) = file::read_buffer(&path)?;
        let signal_chart = SignalChart::new(channels as usize, buffer.len());

        Ok(App {
            buffer: buffer,
            channels: channels,
            name: format!("Sampitor: {}", file_name),
            sample_rate: sample_rate,
            signal_chart: signal_chart,
            sink: sink,
            _stream: stream,
            terminal: terminal::take()?,
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
        self.signal_chart.update(&self.buffer);

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

        terminal::leave(&mut self.terminal)?;

        Ok(())
    }

    fn update(&mut self) {}
}
