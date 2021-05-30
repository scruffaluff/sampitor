//! Terminal oriented functions.

use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub type CrossTerm = Terminal<CrosstermBackend<Stdout>>;

/// Return control of current terminal window from alternate screen.
///
/// # Errors
///
/// Will return `Err` if `terminal` control cannot be returned.
pub fn leave(terminal: &mut CrossTerm) -> eyre::Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    Ok(())
}

/// Take control of current terminal window as an alternate screen.
///
/// # Errors
///
/// Will return `Err` if unable to take control of the current terminal.
pub fn take() -> eyre::Result<CrossTerm> {
    terminal::enable_raw_mode()?;
    let mut screen = io::stdout();
    execute!(screen, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(screen);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
