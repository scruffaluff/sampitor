use crate::action::Action;
use crate::buffer::SamplesBuffer;
use crate::chart::SignalChart;
use crate::event;
use crate::file::File;
use crate::menu::Menu;
use crate::path;
use crate::terminal::{self, CrossTerm};
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use rodio::buffer;
use rodio::{OutputStream, Sink};
use std::env;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};

pub struct App {
    actions: Vec<Box<dyn Action>>,
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
        let name = format!("File: {}", path::name(&path)?);
        let options = vec![String::from("Chart"), String::from("File")];

        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        let samples = path::read_samples(&path)?;
        let channels = samples.channels as usize;

        let chart = SignalChart::new(name, channels, samples.data.len() / channels);
        let file = File::try_new(env::current_dir()?)?;

        Ok(App {
            actions: vec![Box::new(chart), Box::new(file)],
            menu: Menu::new(options, String::from("Menu")),
            samples,
            shutdown: false,
            sink,
            _stream: stream,
            terminal: terminal::take()?,
        })
    }

    fn key_event(&mut self, event: KeyEvent) {
        self.menu.key_event(event);
        let action = &mut self.actions[self.menu.get_state()];
        action.key_event(event);

        match event.code {
            KeyCode::Char(' ') => self.play(),
            KeyCode::Char('q') | KeyCode::Esc => {
                self.shutdown = true;
            }
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
        let action = &mut self.actions[self.menu.get_state()];
        let menu = &mut self.menu;

        self.terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            menu.render(frame, chunks[0]);
            action.render(frame, chunks[1]);
        })?;

        Ok(())
    }

    /// Update internal state.
    fn process(&mut self) {
        for action in self.actions.iter_mut() {
            action.process(&mut self.samples);
        }
    }

    /// Loop and wait for user input.
    pub fn run(&mut self) -> eyre::Result<()> {
        let (sender, receiver) = mpsc::channel::<Option<KeyEvent>>();
        let _ = event::event_thread(sender);

        while !self.shutdown {
            self.process();
            self.render()?;

            match receiver.try_recv() {
                Ok(Some(key_event)) => self.key_event(key_event),
                Ok(None) | Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected.into()),
            }
        }

        terminal::leave(&mut self.terminal)?;

        Ok(())
    }
}
