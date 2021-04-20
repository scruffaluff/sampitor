use crate::action::{Action, CrossFrame};
use crate::buffer::SamplesBuffer;
use crate::file;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::PathBuf;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};

pub struct Reader {
    cwd: PathBuf,
    files: Vec<(String, bool)>,
    read: bool,
    state: ListState,
}

impl Reader {
    pub fn try_new(cwd: PathBuf) -> eyre::Result<Self> {
        let files = file::sorted_names(&cwd)?;

        Ok(Reader {
            cwd,
            files,
            read: false,
            state: ListState::default(),
        })
    }

    fn chdir(&mut self, cwd: PathBuf) {
        self.cwd = cwd;
        self.files = file::sorted_names(&self.cwd)
            .unwrap_or_else(|error| vec![(format!("{}", error), false)]);
        self.read = false;
        self.state = ListState::default();
    }

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

impl Action for Reader {
    fn key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => self.next(),
            KeyCode::Enter => {
                if let Some(index) = self.state.selected() {
                    let (_name, is_dir) = &self.files[index];

                    if !*is_dir {
                        self.read = true;
                    }
                };
            }
            KeyCode::Left => {
                let option = self.cwd.parent().map(|path_ref| path_ref.to_owned());
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

    fn process(&mut self, samples: &mut SamplesBuffer) {
        if self.read {
            if let Some(index) = self.state.selected() {
                let (name, _is_dir) = &self.files[index];
                let path = self.cwd.join(name);

                match file::read_samples(&path) {
                    Ok(buffer) => *samples = buffer,
                    Err(error) => self.files = vec![(format!("{}", error), false)],
                };
            };

            self.read = false;
        }
    }

    fn render(&mut self, frame: &mut CrossFrame, area: Rect) {
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
    }
}
