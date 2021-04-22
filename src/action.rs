use crate::buffer::SamplesBuffer;
use crossterm::event::KeyEvent;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::terminal::Frame;

pub type CrossFrame<'a> = Frame<'a, CrosstermBackend<Stdout>>;

/// Base requirements for a UI view.
pub trait Action {
    /// Update view state based on keyboard input.
    fn key_event(&mut self, event: KeyEvent);
    /// Get or set the current signal state.
    fn process(&mut self, samples: &mut SamplesBuffer);
    /// Draw UI view in area of given frame.
    fn render(&mut self, frame: &mut CrossFrame, area: Rect);
}
