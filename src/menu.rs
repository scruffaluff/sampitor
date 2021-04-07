use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, List, ListItem, ListState};

pub struct Menu {
    options: Vec<String>,
    state: ListState,
    title: String,
}

impl Menu {
    pub fn new(options: Vec<String>, title: String) -> Self {
        Menu {
            options,
            state: ListState::default(),
            title,
        }
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => self.next(),
            KeyCode::Up => self.previous(),
            _ => (),
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.options.len() - 1 {
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
                    self.options.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect, highlight: bool) {
        let options: Vec<ListItem> = self
            .options
            .iter()
            .map(|option| ListItem::new(option.as_ref()))
            .collect();

        let mut block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);

        if highlight {
            block = block.border_style(Style::default().add_modifier(Modifier::BOLD));
        }

        let list = List::new(options)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;

    #[test]
    fn key_event() {
        let mut menu = Menu::new(
            ["A", "set", "of", "options"]
                .iter()
                .map(|string| String::from(*string))
                .collect(),
            String::from("Menu"),
        );

        menu.key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        menu.key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));

        assert_eq!(Some(3), menu.state.selected());
    }
}
