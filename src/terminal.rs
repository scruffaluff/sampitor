use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use std::io;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub type CrossTerm = Terminal<CrosstermBackend<Stdout>>;

/// Return control of current terminal window from alternate screen.
pub fn leave(terminal: &mut CrossTerm) -> eyre::Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    Ok(())
}

/// Take control of current terminal window as an alternate screen.
pub fn take() -> eyre::Result<CrossTerm> {
    terminal::enable_raw_mode()?;
    let mut screen = io::stdout();
    execute!(screen, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(screen);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}
