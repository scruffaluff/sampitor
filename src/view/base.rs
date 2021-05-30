//! Fundemental traits for user interface components.

use crate::dsp::Samples;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

/// Base requirements for a UI view.
pub trait View<B: Backend> {
    /// Update view state based on keyboard input.
    fn key_event(&mut self, event: KeyEvent);
    /// Get or set the current signal state.
    fn process(&mut self, samples: &mut Samples);
    /// Draw UI view in area of given frame.
    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect);
}
