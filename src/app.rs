use crate::buffer::SamplesBuffer;
use crate::chart::SignalChart;
use crate::file;
use crate::menu::Menu;
use crate::terminal::{self, CrossTerm};
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode};
use rodio::buffer;
use rodio::{OutputStream, Sink};
use std::io;
use std::path::PathBuf;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};

pub struct App {
    menu: Menu,
    samples: SamplesBuffer,
    signal_chart: SignalChart<'static>,
    sink: Sink,
    // If stream is dropped then sound will not reach the speakers.
    _stream: OutputStream,
    terminal: CrossTerm,
}

impl App {
    /// Attempt to generate a new App.
    pub fn try_new(path: PathBuf) -> eyre::Result<Self> {
        let name = format!("File: {}", file::name(&path)?);
        let options = vec![
            String::from("Filter"),
            String::from("Read"),
            String::from("Write"),
        ];

        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        let samples = file::read_samples(&path)?;
        let channels = samples.channels as usize;
        let signal_chart = SignalChart::new(name, channels, samples.data.len() / channels);

        Ok(App {
            menu: Menu::new(options, String::from("Menu")),
            samples,
            signal_chart,
            sink,
            _stream: stream,
            terminal: terminal::take()?,
        })
    }

    /// Render all UI elements in terminal screen.
    fn draw(&mut self) -> io::Result<()> {
        let chart = self.signal_chart.render();
        let menu = &mut self.menu;
        let terminal = &mut self.terminal;

        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            menu.render(frame, chunks[0]);
            frame.render_widget(chart, chunks[1]);
        })?;

        Ok(())
    }

    /// Play currently loaded sample.
    fn play(&self) {
        let source = buffer::SamplesBuffer::from(&self.samples);
        self.sink.append(source)
    }

    /// Loop and wait for user input.
    pub fn run(&mut self) -> eyre::Result<()> {
        self.signal_chart.update(&self.samples.data);

        loop {
            self.draw()?;

            if let Event::Key(event) = event::read()? {
                match event.code {
                    KeyCode::Char(' ') => {
                        self.play();
                    }
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down => {
                        self.menu.next();
                    }
                    KeyCode::Up => {
                        self.menu.previous();
                    }
                    _ => (),
                }
            }

            self.update();
        }

        terminal::leave(&mut self.terminal)?;

        Ok(())
    }

    /// Update internal state.
    fn update(&mut self) {}
}
