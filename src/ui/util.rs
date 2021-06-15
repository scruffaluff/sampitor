//! Utilities for drawing visuals.

use tui::layout::{Constraint, Direction, Layout, Rect};

/// Create a centered interior rectangle to a given rectangle.
#[must_use]
pub fn centered_rectangle(percent_x: u16, percent_y: u16, rectangle: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(rectangle);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_dimensions() {
        let rectangle = Rect::new(0, 0, 80, 120);
        let expected = Rect::new(28, 48, 24, 24);

        let actual = centered_rectangle(30, 20, rectangle);
        assert_eq!(actual, expected);
    }
}
