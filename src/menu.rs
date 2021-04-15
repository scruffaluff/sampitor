use tui::style::{Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, Tabs};

pub struct Menu {
    options: Vec<String>,
    state: usize,
    title: String,
}

impl Menu {
    pub fn new(options: Vec<String>, title: String) -> Self {
        Menu {
            options,
            state: 0,
            title,
        }
    }

    pub fn get_state(&self) -> usize {
        self.state
    }

    pub fn next(&mut self) {
        self.state = (self.state + 1) % self.options.len();
    }

    pub fn widget(&self) -> Tabs {
        let options: Vec<Spans> = self
            .options
            .iter()
            .map(|option| Spans::from(option.as_ref()))
            .collect();

        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);

        Tabs::new(options)
            .select(self.state)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
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
