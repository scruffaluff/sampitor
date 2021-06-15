//! Application runners.

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
    pub fn new(views: &'a mut [(&'a str, &'a mut dyn View<B>)], samples: Samples) -> Self {
        Self {
            samples,
            shutdown: false,
            state: 0,
            views,
        }
    }

    /// Pass keyboard input to current view.
    pub fn key_event(&mut self, sink: &Sink, event: KeyEvent) {
        if let Some(view) = self.views.get_mut(self.state) {
            view.1.key_event(event);
        }

        match event.code {
            KeyCode::Char(' ') => self.play(sink),
            KeyCode::Char('q') | KeyCode::Esc => {
                self.shutdown = true;
            }
            KeyCode::Tab => self.next(),
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
        terminal.draw(|frame| {
            let size = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Percentage(16), Percentage(84)].as_ref())
                .split(size);

            self.render_menu(frame, chunks[0]);

            if let Some(view) = self.views.get_mut(self.state) {
                view.1.render(frame, chunks[1]);
            }
        })?;

        Ok(())
    }

    fn render_menu<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect) {
        let options: Vec<Spans> = self.views.iter().map(|view| Spans::from(view.0)).collect();

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
    use crate::util::test::MockView;
    use crossterm::event::KeyModifiers;
    use rodio::Sink;
    use tui::backend::TestBackend;

    #[test]
    fn menu_contains_views() {
        let backend = TestBackend::new(20, 10);
        let mut terminal = Terminal::new(backend).unwrap();

        let mut app = App::new(&mut [], Samples::default());
        app.render(&mut terminal).unwrap();

        let actual = util::test::buffer_view(terminal.backend().buffer());
        assert!(actual.contains("Menu"));
    }

    #[test]
    fn menu_key_event() {
        let sink = Sink::new_idle().0;

        let mut mock1 = MockView::default();
        let mut mock2 = MockView::default();
        let mut mock3 = MockView::default();

        let mut views: Vec<(&str, &mut dyn View<TestBackend>)> = Vec::new();
        views.push(("", &mut mock1));
        views.push(("", &mut mock2));
        views.push(("", &mut mock3));

        let mut app = App::new(&mut views, Samples::default());
        (0..7).for_each(|_| {
            app.key_event(&sink, KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
        });

        assert_eq!(1, app.state);
    }
}
