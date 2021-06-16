//! Components for fitlering signals.

pub mod base;
mod knobs;
pub mod normalize;

pub use base::{Filter, Knob};
pub use normalize::Normalize;

use crate::dsp::Samples;
use crate::view::View;
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::Constraint::{Length, Percentage};
use tui::layout::{Direction, Layout, Rect};
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, List, ListItem, ListState, Row, Table};

/// A UI view for navigating the file system, reading audio files, and writing audio files.
pub struct Filters<'a> {
    filters: &'a mut [(&'a str, &'a mut dyn Filter)],
    mode: Mode,
    filter_state: ListState,
    knob_state: usize,
}

impl<'a> Filters<'a> {
    /// Attempt to crate a File view
    #[must_use]
    pub fn new(filters: &'a mut [(&'a str, &'a mut dyn Filter)]) -> Self {
        Self {
            filters,
            mode: Mode::Nagivate,
            filter_state: ListState::default(),
            knob_state: 0,
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
    fn key_event_edit(&mut self, event: KeyEvent) {
        if let Some(index) = self.filter_state.selected() {
            let knobs = &mut self.filters[index].1.knobs();
            let knob: &mut dyn Knob = knobs[self.knob_state].1;

            match event.code {
                KeyCode::Down => knob.decrement(),
                KeyCode::Up => knob.increment(),
                _ => (),
            }
        }
    }

    /// Modular move list state to next inode.
    fn next(&mut self) {
        let index = match self.filter_state.selected() {
            Some(index) => {
                if index >= self.filters.len() - 1 {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        };
        self.filter_state.select(Some(index));
    }

    /// Modular move list state to previous inode.
    fn previous(&mut self) {
        let index = match self.filter_state.selected() {
            Some(index) => {
                if index == 0 {
                    self.filters.len() - 1
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.filter_state.select(Some(index));
    }
}

impl<'a, B: Backend> View<B> for Filters<'a> {
    fn key_event(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Edit => self.key_event_edit(event),
            Mode::Filter => {}
            Mode::Nagivate => self.key_event_navigate(event),
        }
    }

    fn process(&mut self, _samples: &mut Samples) -> eyre::Result<()> {
        if self.mode == Mode::Filter {
            self.mode = Mode::Nagivate;
        }

        Ok(())
    }

    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Percentage(16), Percentage(84)].as_ref())
            .split(area);

        let entries: Vec<ListItem> = self
            .filters
            .iter()
            .map(|filter| ListItem::new(filter.0))
            .collect();

        let block = Block::default().borders(Borders::ALL);

        let list = List::new(entries)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, chunks[0], &mut self.filter_state);

        if let Some(index) = self.filter_state.selected() {
            let knobs = self.filters[index].1.knobs();
            let rows: Vec<Row> = knobs
                .iter()
                .map(|(name, filter)| Row::new(vec![*name, filter.text()]))
                .collect();

            let block = Block::default().borders(Borders::ALL);

            let table = Table::new(rows)
                .header(Row::new(vec!["Knob", "Value"]))
                .block(block)
                .widths(&[Length(12), Length(12)]);
            frame.render_widget(table, chunks[1]);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Edit,
    Filter,
    Nagivate,
}
