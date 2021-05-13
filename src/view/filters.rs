use crate::dsp::Samples;
use crate::view::View;
use crossterm::event::{KeyCode, KeyEvent};
use std::cmp::Ordering;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, List, ListItem, ListState};

/// A UI view for navigating the file system, reading audio files, and writing audio files.
pub struct Filters {
    filters: Vec<String>,
    mode: Mode,
    state: ListState,
}

impl Filters {
    /// Attempt to crate a File view
    #[must_use]
    pub fn new(filters: Vec<String>) -> Self {
        Self {
            filters,
            mode: Mode::Nagivate,
            state: ListState::default(),
        }
    }

    /// Handle key events while in navigate mode.
    fn key_event_navigate(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('e') => self.mode = Mode::Edit,
            KeyCode::Down => self.next(),
            KeyCode::Enter => self.mode = Mode::Filter,
            KeyCode::Up => self.previous(),
            _ => (),
        }
    }

    /// Handle key events while in type mode.
    fn key_event_edit(&mut self, _event: KeyEvent) {
        unimplemented!()
    }

    /// Modular move list state to next inode.
    fn next(&mut self) {
        let index = match self.state.selected() {
            Some(index) => {
                if index >= self.filters.len() - 1 {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }

    /// Modular move list state to previous inode.
    fn previous(&mut self) {
        let index = match self.state.selected() {
            Some(index) => {
                if index == 0 {
                    self.filters.len() - 1
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }
}

impl<B: Backend> View<B> for Filters {
    fn key_event(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Edit => self.key_event_edit(event),
            Mode::Filter => {}
            Mode::Nagivate => self.key_event_navigate(event),
        }
    }

    fn process(&mut self, samples: &mut Samples) {
        if self.mode == Mode::Filter {
            let maximum = samples
                .data
                .iter()
                .map(|x| f32::abs(*x))
                .max_by(|x, y| x.partial_cmp(y).unwrap_or(Ordering::Equal));

            if let Some(maximum) = maximum {
                samples.data.iter_mut().for_each(|x| *x /= maximum);
            }

            self.mode = Mode::Nagivate;
        }
    }

    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect) {
        let entries: Vec<ListItem> = self
            .filters
            .iter()
            .map(|filter| ListItem::new(filter.as_ref()))
            .collect();

        let block = Block::default().borders(Borders::ALL);

        let list = List::new(entries)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Edit,
    Filter,
    Nagivate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;
    use tui::backend::TestBackend;

    #[test]
    fn key_event() {
        let mut filters = Filters::new(vec![String::from("Normalize")]);
        let mut actual = Samples::new(2, 20, vec![-0.5, -0.25, 0.25, 0.0]);

        View::<TestBackend>::key_event(
            &mut filters,
            KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE),
        );
        View::<TestBackend>::process(&mut filters, &mut actual);

        let expected = Samples::new(2, 20, vec![-1.0, -0.5, 0.5, 0.0]);
        assert_eq!(actual.data, expected.data);
    }
}
