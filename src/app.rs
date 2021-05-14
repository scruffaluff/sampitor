use crate::dsp::Samples;
use crate::io::event;
use crate::view::View;
use color_eyre::eyre;
use crossterm::event::{KeyCode, KeyEvent};
use rodio::buffer::SamplesBuffer;
use rodio::Sink;
use std::sync::mpsc::{self, TryRecvError};
use tui::backend::Backend;
use tui::layout::Constraint::Percentage;
use tui::layout::{Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::terminal::{Frame, Terminal};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Tabs};

/// Main runner for Sampitor application.
pub struct App<'a, B: Backend> {
    samples: Samples,
    shutdown: bool,
    state: usize,
    views: &'a mut [(&'a str, &'a mut dyn View<B>)],
}

impl<'a, B: Backend> App<'a, B> {
    /// Create a new App.
    pub fn new(views: &'a mut [(&'a str, &'a mut dyn View<B>)]) -> Self {
        Self {
            samples: Samples::default(),
            shutdown: false,
            state: 0,
            views,
        }
    }

    /// Pass keyboard input to current view.
    pub fn key_event(&mut self, sink: &Sink, event: KeyEvent) {
        let view = &mut self.views[self.state].1;
        view.key_event(event);

        match event.code {
            KeyCode::Char(' ') => self.play(sink),
            KeyCode::Char('q') | KeyCode::Esc => {
                self.shutdown = true;
            }
            _ => (),
        }
    }

    /// Modular move menu state to next option.
    pub fn next(&mut self) {
        self.state = (self.state + 1) % self.views.len();
    }

    /// Play currently loaded signal.
    pub fn play(&self, sink: &Sink) {
        let source = SamplesBuffer::from(&self.samples);
        sink.append(source)
    }

    /// Update internal signal state.
    pub fn process(&mut self) {
        for (_name, view) in &mut self.views.iter_mut() {
            view.process(&mut self.samples);
        }
    }

    /// Render all UI views in terminal screen.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `terminal` cannot draw frames.
    pub fn render(&mut self, terminal: &mut Terminal<B>) -> eyre::Result<()> {
        let view = &mut self.views[self.state].1;

        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            // self.render_menu(frame, chunks[0]);
            view.render(frame, chunks[1]);
        })?;

        Ok(())
    }

    fn render_menu<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect) {
        let options: Vec<Spans> = self
            .views
            .iter()
            .map(|view| Spans::from(view.0.as_ref()))
            .collect();

        let block = Block::default().title("Menu").borders(Borders::ALL);

        let tabs = Tabs::new(options)
            .select(self.state)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
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

        let mut app = App::new(&[]);
        app.render(&mut terminal).unwrap();

        let expected = "Menu";

        let actual = util::test::buffer_view(terminal.backend().buffer());
        assert!(actual.contains(expected));
    }

    #[test]
    fn menu_key_event() {
        let mut menu = Menu::new(
            ["A", "set", "of", "options"]
                .iter()
                .map(|string| String::from(*string))
                .collect(),
            String::from("Menu"),
        );

        View::<TestBackend>::key_event(&mut menu, KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        View::<TestBackend>::key_event(&mut menu, KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));

        assert_eq!(2, menu.get_state());
    }
}
