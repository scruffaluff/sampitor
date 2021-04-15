use crossterm::event::KeyEvent;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::terminal::Frame;

pub type CrossFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait Action {
    fn key_event(&mut self, event: KeyEvent);
    fn render(&self, frame: &mut CrossFrame, area: Rect);
    fn update(&mut self, buffer: &[f32]);
}
