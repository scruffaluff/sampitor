use color_eyre::eyre;
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Stdout;
use std::path::PathBuf;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders};
use tui::Terminal;

type CrossTerm = Terminal<CrosstermBackend<Stdout>>;

pub struct App {
    name: String,
    path: PathBuf,
    sink: Sink,
    // If stream is dropped then sound will not reach the speakers.
    _stream: OutputStream,
    terminal: CrossTerm,
}

impl App {
    pub fn try_new(path: PathBuf) -> eyre::Result<Self> {
        let file_name = path
            .file_name()
            .ok_or(eyre::eyre!(
                "File path {:?} does not have a final component",
                path
            ))?
            .to_str()
            .ok_or(eyre::eyre!("File name {:?} is not valid Unicode", path))?;

        let (stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;

        Ok(App {
            name: format!("Sampitor: {}", file_name),
            path: path,
            sink: sink,
            _stream: stream,
            terminal: take_terminal()?,
        })
    }

    fn draw(&mut self) -> io::Result<()> {
        let name: &str = &self.name;
        self.terminal.draw(|frame| {
            let size = frame.size();
            let block = Block::default().title(name).borders(Borders::ALL);
            frame.render_widget(block, size);
        })?;

        Ok(())
    }

    fn play(&self) -> eyre::Result<()> {
        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let source = Decoder::new(reader)?;
        Ok(self.sink.append(source))
    }

    pub fn run(&mut self) -> eyre::Result<()> {
        loop {
            self.draw()?;

            match event::read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        self.play()?;
                    }
                    _ => (),
                },
                _ => (),
            }

            self.update();
        }

        leave_terminal(&mut self.terminal)?;

        Ok(())
    }

    fn update(&mut self) {}
}

fn take_terminal() -> eyre::Result<CrossTerm> {
    terminal::enable_raw_mode()?;
    let mut screen = io::stdout();
    execute!(screen, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(screen);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn leave_terminal(terminal: &mut CrossTerm) -> eyre::Result<()> {
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;

    Ok(())
}
