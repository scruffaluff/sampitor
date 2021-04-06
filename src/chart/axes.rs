use crossterm::event::{KeyCode, KeyEvent};
use tui::widgets::Axis;

pub struct Axes {
    x: [f64; 2],
    y: [f64; 2],
    _step: f64,
}

impl Axes {
    pub fn new(x: [f64; 2], y: [f64; 2]) -> Self {
        Self {
            x,
            y,
            _step: (x[1] - x[0]) / 10.0,
        }
    }

    pub fn axes(&self) -> (Axis, Axis) {
        (
            Axis::default().bounds(self.x),
            Axis::default().bounds(self.y),
        )
    }

    pub fn key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Down => (),
            KeyCode::Left => (),
            KeyCode::Right => (),
            KeyCode::Up => (),
            _ => (),
        }
    }
}
