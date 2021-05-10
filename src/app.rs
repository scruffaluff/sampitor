use crate::dsp::buffer::SamplesBuffer;
use crate::io::{event, path};
use crate::view::{File, Menu, SignalChart, View};
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use rodio::buffer;
use rodio::{OutputStream, Sink};
use std::env;
use std::path::PathBuf;
use std::sync::mpsc::{self, TryRecvError};
use tui::backend::Backend;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};
use tui::terminal::Terminal;

/// Main runner for Sampitor application.
pub struct App<B: Backend> {
    menu: Menu,
    samples: SamplesBuffer,
    shutdown: bool,
    sink: Sink,
    // If stream is dropped then sound will not reach the speakers.
    _stream: OutputStream,
    views: Vec<Box<dyn View<B>>>,
}

impl<B: Backend> App<B> {
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
            menu: Menu::new(options, String::from("Menu")),
            samples,
            shutdown: false,
            sink,
            _stream: stream,
            views: vec![Box::new(chart), Box::new(file)],
        })
    }

    /// Pass keyboard input to current view.
    pub fn key_event(&mut self, event: KeyEvent) {
        View::<B>::key_event(&mut self.menu, event);
        let view = &mut self.views[self.menu.get_state()];
        view.key_event(event);

        match event.code {
            KeyCode::Char(' ') => self.play(),
            KeyCode::Char('q') | KeyCode::Esc => {
                self.shutdown = true;
            }
            _ => (),
        }
    }

    /// Play currently loaded signal.
    pub fn play(&self) {
        let source = buffer::SamplesBuffer::from(&self.samples);
        self.sink.append(source)
    }

    /// Render all UI views in terminal screen.
    pub fn render(&mut self, terminal: &mut Terminal<B>) -> eyre::Result<()> {
        let view = &mut self.views[self.menu.get_state()];
        let menu = &mut self.menu;

        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            menu.render(frame, chunks[0]);
            view.render(frame, chunks[1]);
        })?;

        Ok(())
    }

    /// Update internal signal state.
    pub fn process(&mut self) {
        for view in self.views.iter_mut() {
            view.process(&mut self.samples);
        }
    }

    /// Loop and wait for user keyboard input.
    pub fn run(&mut self, terminal: &mut Terminal<B>) -> eyre::Result<()> {
        let (sender, receiver) = mpsc::channel::<Option<KeyEvent>>();
        let _ = event::event_thread(sender);

        while !self.shutdown {
            self.process();
            self.render(terminal)?;

            match receiver.try_recv() {
                Ok(Some(key_event)) => self.key_event(key_event),
                Ok(None) | Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected.into()),
            }
        }

        Ok(())
    }
}
