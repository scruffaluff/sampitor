use crate::dsp::SamplesBuffer;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

/// Base requirements for a UI view.
pub trait View<B: Backend> {
    /// Update view state based on keyboard input.
    fn key_event(&mut self, event: KeyEvent);
    /// Get or set the current signal state.
    fn process(&mut self, samples: &mut SamplesBuffer);
    /// Draw UI view in area of given frame.
    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect);
}
