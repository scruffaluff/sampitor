use crate::buffer::SamplesBuffer;
use crate::chart::SignalChart;
use crate::file;
use crate::menu::Menu;
use crate::terminal::{self, CrossTerm};
use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use rodio::buffer;
use rodio::{OutputStream, Sink};
use std::io;
use std::path::PathBuf;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};

pub struct App {
    chart: SignalChart<'static>,
    menu: Menu,
    samples: SamplesBuffer,
    selection: usize,
    shutdown: bool,
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
        let chart = SignalChart::new(name, channels, samples.data.len() / channels);

        Ok(App {
            chart,
            menu: Menu::new(options, String::from("Menu")),
            samples,
            selection: 0,
            shutdown: false,
            sink,
            _stream: stream,
            terminal: terminal::take()?,
        })
    }

    /// Render all UI elements in terminal screen.
    fn draw(&mut self) -> io::Result<()> {
        let chart = &mut self.chart;
        let menu = &mut self.menu;
        let selection = self.selection;

        self.terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            menu.render(frame, chunks[0], selection == 0);
            chart.render(frame, chunks[1], selection == 1);
        })?;

        Ok(())
    }

    fn key_event(&mut self, event: KeyEvent) {
        match self.selection {
            0 => self.menu.key_event(event),
            1 => self.chart.key_event(event),
            _ => (),
        };

        match event.code {
            KeyCode::Char(' ') => {
                self.play();
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.shutdown = true;
            }
            KeyCode::Tab => {
                self.selection = match self.selection {
                    0 => 1,
                    1 => 0,
                    _ => 0,
                };
            }
            _ => (),
        }
    }

    /// Play currently loaded sample.
    fn play(&self) {
        let source = buffer::SamplesBuffer::from(&self.samples);
        self.sink.append(source)
    }

    /// Loop and wait for user input.
    pub fn run(&mut self) -> eyre::Result<()> {
        // TODO: Move to update method with logic should update logic.
        self.chart.update(&self.samples.data);

        loop {
            self.draw()?;
            if let Event::Key(key) = event::read()? {
                self.key_event(key);
            };
            self.update();

            if self.shutdown {
                break;
            }
        }

        terminal::leave(&mut self.terminal)?;

        Ok(())
    }

    /// Update internal state.
    fn update(&mut self) {}
}
