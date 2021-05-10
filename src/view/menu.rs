use crate::dsp::buffer::SamplesBuffer;
use crate::view::View;
use crossterm::event::{KeyCode, KeyEvent};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Modifier, Style};
use tui::terminal::Frame;
use tui::text::Spans;
use tui::widgets::{Block, Borders, Tabs};

/// UI view bar for selecting other UI views.
pub struct Menu {
    options: Vec<String>,
    state: usize,
    title: String,
}

impl Menu {
    /// Create a new Menu from a title and options.
    pub fn new(options: Vec<String>, title: String) -> Self {
        Menu {
            options,
            state: 0,
            title,
        }
    }

    /// Get the interior menu state for rendering.
    pub fn get_state(&self) -> usize {
        self.state
    }

    /// Modular move menu state to next option.
    pub fn next(&mut self) {
        self.state = (self.state + 1) % self.options.len();
    }
}

impl<B: Backend> View<B> for Menu {
    fn key_event(&mut self, event: KeyEvent) {
        if event.code == KeyCode::Tab {
            self.next()
        }
    }

    fn process(&mut self, _samples: &mut SamplesBuffer) {}

    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect) {
        let options: Vec<Spans> = self
            .options
            .iter()
            .map(|option| Spans::from(option.as_ref()))
            .collect();

        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);

        let tabs = Tabs::new(options)
            .select(self.state)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(tabs, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyModifiers;
    use tui::backend::TestBackend;

    #[test]
    fn key_event() {
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
