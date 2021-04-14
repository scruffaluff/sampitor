use crate::buffer::SamplesBuffer;
use crate::chart::SignalChart;
use crate::event;
use crate::file;
use crate::menu::Menu;
use crate::terminal::{self, CrossTerm};
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use rodio::buffer;
use rodio::{OutputStream, Sink};
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};

pub struct App {
    actions: Vec<Menu>,
    chart: SignalChart<'static>,
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
            actions: vec![Menu::new(options, String::from("Menu"))],
            chart,
            samples,
            selection: 0,
            shutdown: false,
            sink,
            _stream: stream,
            terminal: terminal::take()?,
        })
    }

    fn key_event(&mut self, event: KeyEvent) -> eyre::Result<()> {
        let action = self
            .actions
            .first_mut()
            .ok_or_else(|| eyre::eyre!("No actions available"))?;
        match self.selection {
            0 => action.key_event(event),
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

        Ok(())
    }

    /// Play currently loaded sample.
    fn play(&self) {
        let source = buffer::SamplesBuffer::from(&self.samples);
        self.sink.append(source)
    }

    /// Render all UI elements in terminal screen.
    fn render(&mut self) -> eyre::Result<()> {
        let chart = &mut self.chart;
        let action = self
            .actions
            .first_mut()
            .ok_or_else(|| eyre::eyre!("No actions available"))?;
        let selection = self.selection;

        self.terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            action.render(frame, chunks[0], selection == 0);
            chart.render(frame, chunks[1], selection == 1);
        })?;

        Ok(())
    }

    /// Loop and wait for user input.
    pub fn run(&mut self) -> eyre::Result<()> {
        // TODO: Move to update method with should update logic.
        self.chart.update(&self.samples.data);

        let (sender, receiver) = mpsc::channel::<Option<KeyEvent>>();
        let _ = event::event_thread(sender);

        while !self.shutdown {
            self.render()?;

            match receiver.try_recv() {
                Ok(Some(key_event)) => self.key_event(key_event)?,
                Ok(None) | Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected.into()),
            }

            self.update();
        }

        terminal::leave(&mut self.terminal)?;

        Ok(())
    }

    /// Update internal state.
    fn update(&mut self) {}
}
