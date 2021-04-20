use crate::action::{Action, CrossFrame};
use crate::file;
use crossterm::event::{KeyCode, KeyEvent};
use std::path::PathBuf;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};

pub struct Reader {
    _cwd: PathBuf,
    files: Vec<(String, bool)>,
    state: ListState,
}

impl Reader {
    pub fn try_new(cwd: PathBuf) -> eyre::Result<Self> {
        let files = file::sorted_names(&cwd)?;

        Ok(Reader {
            _cwd: cwd,
            files,
            state: ListState::default(),
        })
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.files.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.files.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Action for Reader {
    fn key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => self.next(),
            KeyCode::Enter => (),
            KeyCode::Up => self.previous(),
            _ => (),
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

    fn update(&mut self, _buffer: &[f32]) {}
}
