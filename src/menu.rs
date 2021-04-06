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

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    pub fn render<B: Backend>(&mut self, frame: &mut Frame<B>, area: Rect) {
        let options: Vec<ListItem> = self
            .options
            .iter()
            .map(|option| ListItem::new(option.as_ref()))
            .collect();

        let list = List::new(options)
            .block(
                Block::default()
                    .title(self.title.as_str())
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(list, area, &mut self.state);
    }
}
