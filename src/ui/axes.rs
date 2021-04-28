use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::text::Span;
use tui::widgets::Axis;

/// Chart axes with shift and zoom features..
pub struct Axes {
    speed: f64,
    x: [f64; 2],
    y: [f64; 2],
}

impl Axes {
    /// Create an Axes from viewport dimensions.
    pub fn new(x: [f64; 2], y: [f64; 2], speed: f64) -> Self {
        Self { speed, x, y }
    }

    /// Generate a TUI Axis pair.
    pub fn axes(&self) -> (Axis, Axis) {
        let labels: (Vec<Span>, Vec<Span>) = (
            self.x
                .iter()
                .map(|num| Span::from(format!("{:.2}", num)))
                .collect(),
            self.y
                .iter()
                .map(|num| Span::from(format!("{:.2}", num)))
                .collect(),
        );

        (
            Axis::default().bounds(self.x).labels(labels.0),
            Axis::default().bounds(self.y).labels(labels.1),
        )
    }

    /// Update axes state based on keyboard input.
    pub fn key_event(&mut self, event: KeyEvent) {
        match event.modifiers {
            KeyModifiers::SHIFT => self.zoom(event.code),
            _ => self.shift(event.code),
        }

        match event.code {
            KeyCode::Char('+') => self.speed *= 2.0,
            KeyCode::Char('-') => self.speed *= 0.5,
            _ => (),
        }
    }

    /// Move axes bounds horizontally or vertically.
    fn shift(&mut self, code: KeyCode) {
        let direction = match code {
            KeyCode::Down => (0.0, -1.0),
            KeyCode::Left => (-1.0, 0.0),
            KeyCode::Right => (1.0, 0.0),
            KeyCode::Up => (-0.0, 1.0),
            _ => return,
        };

        let delta = (
            self.speed * direction.0 * (self.x[1] - self.x[0]) / 10.0,
            self.speed * direction.1 * (self.y[1] - self.y[0]) / 10.0,
        );
        self.x.iter_mut().for_each(|elem| *elem += delta.0);
        self.y.iter_mut().for_each(|elem| *elem += delta.1);
    }

    /// Squeeze or expand axes bounds.
    fn zoom(&mut self, code: KeyCode) {
        let delta = match code {
            KeyCode::Down => 2.0 * self.speed,
            KeyCode::Up => 0.5 * self.speed,
            _ => return,
        };

        let center = [(self.x[1] + self.x[0]) / 2.0, (self.y[1] + self.y[0]) / 2.0];
        let radius = [(self.x[1] - self.x[0]) / 2.0, (self.y[1] - self.y[0]) / 2.0];

        self.x = [center[0] - delta * radius[0], center[0] + delta * radius[0]];
        self.y = [center[1] - delta * radius[1], center[1] + delta * radius[1]];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_left() {
        let mut axes = Axes::new([5.0, 10.0], [-1.0, 1.0], 1.0);
        axes.shift(KeyCode::Left);

        let expected = ([4.5, 9.5], [-1.0, 1.0]);
        let actual = (axes.x, axes.y);

        assert_eq!(actual, expected);
    }

    #[test]
    fn zoom_in() {
        let mut axes = Axes::new([5.0, 10.0], [-1.0, 1.0], 1.0);
        axes.zoom(KeyCode::Up);

        let expected = ([6.25, 8.75], [-0.5, 0.5]);
        let actual = (axes.x, axes.y);

        assert_eq!(actual, expected);
    }
}
