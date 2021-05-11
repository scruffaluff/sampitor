use crate::dsp::Samples;
use crate::io::{event, path};
use crate::view::{File, Menu, Signal, View};
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use rodio::buffer::SamplesBuffer;
use rodio::Sink;
use std::convert::TryInto;
use std::env;
use std::path::Path;
use std::sync::mpsc::{self, TryRecvError};
use tui::backend::Backend;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout};
use tui::terminal::Terminal;

/// Main runner for Sampitor application.
pub struct App<B: Backend> {
    menu: Menu,
    samples: Samples,
    shutdown: bool,
    views: Vec<Box<dyn View<B>>>,
}

impl<B: Backend> App<B> {
    /// Attempt to generate a new App.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `path` does not exist or contains invalid audio data.
    pub fn try_new(path: &Path) -> eyre::Result<Self> {
        let name = format!("File: {}", path::name(path)?);
        let options = vec![String::from("Chart"), String::from("File")];

        let samples = path::read_samples(path)?;
        let channels: usize = samples.channels.try_into()?;

        let chart = Signal::new(name, channels, samples.data.len() / channels);
        let file = File::try_new(env::current_dir()?)?;

        Ok(Self {
            menu: Menu::new(options, String::from("Menu")),
            samples,
            shutdown: false,
            views: vec![Box::new(chart), Box::new(file)],
        })
    }

    /// Pass keyboard input to current view.
    pub fn key_event(&mut self, sink: &Sink, event: KeyEvent) {
        View::<B>::key_event(&mut self.menu, event);
        let view = &mut self.views[self.menu.get_state()];
        view.key_event(event);

        match event.code {
            KeyCode::Char(' ') => self.play(sink),
            KeyCode::Char('q') | KeyCode::Esc => {
                self.shutdown = true;
            }
            _ => (),
        }
    }

    /// Play currently loaded signal.
    pub fn play(&self, sink: &Sink) {
        let source = SamplesBuffer::from(&self.samples);
        sink.append(source)
    }

    /// Render all UI views in terminal screen.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `terminal` cannot draw frames.
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
        for view in &mut self.views {
            view.process(&mut self.samples);
        }
    }

    /// Loop and wait for user keyboard input.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `terminal` cannot draw frames.
    pub fn run(&mut self, terminal: &mut Terminal<B>, sink: &Sink) -> eyre::Result<()> {
        let (sender, receiver) = mpsc::channel::<Option<KeyEvent>>();
        let _thread_handle = event::handler(sender);

        while !self.shutdown {
            self.process();
            self.render(terminal)?;

            match receiver.try_recv() {
                Ok(Some(key_event)) => self.key_event(sink, key_event),
                Ok(None) | Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => return Err(TryRecvError::Disconnected.into()),
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;
    use tui::backend::TestBackend;

    #[test]
    fn menu_contains_views() {
        let samples = Samples::new(2, 32, vec![0.0f32, -0.25f32, 0.25f32, 1.0f32]);
        let file_path = util::test::temp_wave_file(&samples).unwrap();

        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = App::try_new(&file_path).unwrap();
        app.render(&mut terminal).unwrap();

        let expected = "Menu";

        let actual = util::test::buffer_view(terminal.backend().buffer());
        assert!(actual.contains(expected));
    }
}
