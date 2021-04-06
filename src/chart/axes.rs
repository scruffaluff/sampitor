use tui::widgets::Axis;

pub struct Axes {
    x: [f64; 2],
    y: [f64; 2],
    step: f64,
}

impl Axes {
    pub fn new(x: [f64; 2], y: [f64; 2]) -> Self {
        Self {
            x,
            y,
            step: (x[1] - x[0]) / 10.0,
        }
    }

    pub fn axes(&self) -> (Axis, Axis) {
        (
            Axis::default().bounds(self.x),
            Axis::default().bounds([-1.0, 1.0]),
        )
    }
}
