//! Components for navigating file systems.

use crate::dsp::Samples;
use crate::io::{audio, path};
use crate::ui;
use crate::view::View;
use crossterm::event::{KeyCode, KeyEvent};
use std::borrow::ToOwned;
use std::path::PathBuf;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::text::Text;
use tui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph};

/// A UI view for navigating the file system, reading audio files, and writing audio files.
pub struct File {
    cwd: PathBuf,
    files: Vec<(String, bool)>,
    mode: Mode,
    state: ListState,
    type_buffer: String,
}

impl File {
    /// Attempt to create a File view.
    ///
    /// # Errors
    ///
    /// Will return `Err` if `path` does not exist or contains invalid audio data.
    pub fn try_new(cwd: PathBuf) -> eyre::Result<Self> {
        let files = path::sorted_names(&cwd)?;

        Ok(Self {
            cwd,
            files,
            mode: Mode::Nagivate,
            state: ListState::default(),
            type_buffer: String::new(),
        })
    }

    /// Change working directory and load its files.
    fn chdir(&mut self, cwd: PathBuf) {
        self.cwd = cwd;
        self.files = path::sorted_names(&self.cwd)
            .unwrap_or_else(|error| vec![(format!("{}", error), false)]);
        self.mode = Mode::Nagivate;
        self.state = ListState::default();
    }

    /// Handle key events while in navigate mode.
    fn key_event_navigate(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('w') => self.mode = Mode::Type,
            KeyCode::Down => self.next(),
            KeyCode::Enter => {
                if let Some(index) = self.state.selected() {
                    let (_name, is_dir) = &self.files[index];

                    if !*is_dir {
                        self.mode = Mode::Read;
                    }
                };
            }
            KeyCode::Left => {
                let option = self.cwd.parent().map(ToOwned::to_owned);
                if let Some(path_ref) = option {
                    self.chdir(path_ref)
                }
            }
            KeyCode::Right => {
                if let Some(index) = self.state.selected() {
                    let (name, is_dir) = &self.files[index];

                    if *is_dir {
                        let path = self.cwd.join(name);
                        self.chdir(path);
                    }
                };
            }
            KeyCode::Up => self.previous(),
            _ => (),
        }
    }

    /// Handle key events while in type mode.
    fn key_event_type(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Backspace | KeyCode::Delete => {
                self.type_buffer.pop();
            }
            KeyCode::Enter => self.mode = Mode::Write,
            KeyCode::Char(char) => {
                self.type_buffer.push(char);
            }
            _ => (),
        }
    }

    /// Modular move list state to next inode.
    fn next(&mut self) {
        let index = match self.state.selected() {
            Some(index) => {
                if index >= self.files.len() - 1 {
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
                    self.files.len() - 1
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }
}

impl<B: Backend> View<B> for File {
    fn key_event(&mut self, event: KeyEvent) {
        match self.mode {
            Mode::Nagivate => self.key_event_navigate(event),
            Mode::Type => self.key_event_type(event),
            _ => (),
        }
    }

    fn process(&mut self, samples: &mut Samples) -> eyre::Result<()> {
        match self.mode {
            Mode::Read => {
                if let Some(index) = self.state.selected() {
                    let (name, _is_dir) = &self.files[index];
                    let path = self.cwd.join(name);
                    *samples = audio::read_samples(&path)?;
                };

                self.mode = Mode::Nagivate;
            }
            Mode::Write => {
                let path = self.cwd.join(&self.type_buffer);
                audio::write_samples(&path, samples)?;

                self.type_buffer.clear();
                self.chdir(self.cwd.clone());
                self.mode = Mode::Nagivate;
            }
            _ => (),
        }

        Ok(())
    }

    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect) {
        let entries: Vec<ListItem> = self
            .files
            .iter()
            .map(|(file, is_dir)| {
                let item = ListItem::new(file.as_ref());

                if *is_dir {
                    item.style(Style::default().add_modifier(Modifier::BOLD))
                } else {
                    item
                }
            })
            .collect();

        let block = Block::default().borders(Borders::ALL);

        let list = List::new(entries)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut self.state);

        if self.mode == Mode::Type {
            let area = ui::util::centered_rectangle(60, 20, area);
            frame.render_widget(Clear, area);

            let block = Block::default().title("Write").borders(Borders::ALL);
            let text = Text::from(self.type_buffer.as_ref());
            let line = Paragraph::new(text).block(block);

            frame.render_widget(line, area);
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Mode {
    Read,
    Nagivate,
    Type,
    Write,
}

#[cfg(test)]
mod tests {}
