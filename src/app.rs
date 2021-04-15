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
    actions: Vec<SignalChart<'static>>,
    menu: Menu,
    samples: SamplesBuffer,
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
        let options = vec![String::from("Chart")];

        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        let samples = file::read_samples(&path)?;
        let channels = samples.channels as usize;
        let chart = SignalChart::new(name, channels, samples.data.len() / channels);

        Ok(App {
            actions: vec![chart],
            menu: Menu::new(options, String::from("Menu")),
            samples,
            shutdown: false,
            sink,
            _stream: stream,
            terminal: terminal::take()?,
        })
    }

    fn key_event(&mut self, event: KeyEvent) {
        let action = &mut self.actions[self.menu.get_state()];
        action.key_event(event);

        match event.code {
            KeyCode::Char(' ') => {
                self.play();
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                self.shutdown = true;
            }
            KeyCode::Tab => self.menu.next(),
            _ => (),
        }
    }

    /// Play currently loaded sample.
    fn play(&self) {
        let source = buffer::SamplesBuffer::from(&self.samples);
        self.sink.append(source)
    }

    /// Render all UI elements in terminal screen.
    fn render(&mut self) -> eyre::Result<()> {
        let action = self.actions[self.menu.get_state()].widget();
        let menu = self.menu.widget();

        self.terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            frame.render_widget(menu, chunks[0]);
            frame.render_widget(action, chunks[1]);
        })?;

        Ok(())
    }

    /// Loop and wait for user input.
    pub fn run(&mut self) -> eyre::Result<()> {
        // TODO: Move to update method with should update logic.
        self.actions[0].update(&self.samples.data);

        let (sender, receiver) = mpsc::channel::<Option<KeyEvent>>();
        let _ = event::event_thread(sender);

        while !self.shutdown {
            self.render()?;

            match receiver.try_recv() {
                Ok(Some(key_event)) => self.key_event(key_event),
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
