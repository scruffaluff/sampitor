//! Fundemental traits for user interface components.

use crate::dsp::Samples;
use color_eyre::eyre;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

/// Base requirements for a UI view.
pub trait View<B: Backend> {
    /// Update view state based on keyboard input.
    fn key_event(&mut self, event: KeyEvent);
    /// Get or set the current signal state.
    ///
    /// # Errors
    ///
    /// Will return `Err` if unable to process.
    fn process(&mut self, samples: &mut Samples) -> eyre::Result<()>;
    /// Draw UI view in area of given frame.
    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect);
    /// Reset internal state to a non erroneous case.
    fn reset(&mut self);
}
