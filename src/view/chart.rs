//! Components for plotting audio signals.

use crate::dsp::Samples;
use crate::ui::axes::Axes;
use crate::view::View;
use color_eyre::eyre;
use crossterm::event::KeyEvent;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::symbols::Marker;
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Dataset, GraphType};

/// UI view for plotting audio Chart with shift and zoom features.
pub struct Chart<'a> {
    axes: Axes,
    dataset: Dataset<'a>,
    points: Vec<Vec<(f64, f64)>>,
    title: String,
}

impl<'a> Chart<'a> {
    /// Create a new Chart from a title and audio metadata.
    #[must_use]
    pub fn new(title: String, channels: usize, frame_count: usize) -> Self {
        let axes = Axes::new([0.0_f64, frame_count as f64], [-1.0_f64, 1.0_f64], 1.0_f64);

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line);

        let points = (0..frame_count)
            .map(|index| (index as f64, 0.0_f64))
            .collect();

        Chart {
            axes,
            dataset,
            points: vec![points; channels],
            title,
        }
    }
}

impl<'a, B: Backend> View<B> for Chart<'a> {
    fn key_event(&mut self, event: KeyEvent) {
        self.axes.key_event(event);
    }

    fn process(&mut self, buffer: &mut Samples) -> eyre::Result<()> {
        let channels: usize = buffer.channels.into();
        let frame_count = buffer.data.len() / channels;

        let points = (0..frame_count)
            .map(|index| (index as f64, 0.0_f64))
            .collect();
        self.points = vec![points; channels];

        for (outer_index, points) in self.points.iter_mut().enumerate() {
            for (inner_index, element) in points.iter_mut() {
                // Variable inner_index should always be positive, so sign loss should not be
                // possible.
                #[allow(clippy::cast_sign_loss)]
                let index = channels * (*inner_index as usize) + outer_index;
                *element = buffer.data[index].into();
            }
        }

        Ok(())
    }

    fn render<'b>(&mut self, frame: &mut Frame<'b, B>, area: Rect) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);

        let datasets = self
            .points
            .iter()
            .map(|points| self.dataset.clone().data(points))
            .collect();

        let (x_axis, y_axis) = self.axes.axes();
        let chart = tui::widgets::Chart::new(datasets)
            .block(block)
            .x_axis(x_axis)
            .y_axis(y_axis);

        frame.render_widget(chart, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui::backend::TestBackend;

    #[test]
    fn new_points() {
        let chart = Chart::new(String::from(""), 2, 3);
        let expected = vec![
            vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
            vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
        ];

        assert_eq!(chart.points, expected);
    }

    #[test]
    fn process_points() {
        let mut chart = Chart::new(String::from(""), 1, 1);
        let axes = chart.axes.clone();
        let expected = vec![
            vec![(0.0, -1.0), (1.0, -0.25), (2.0, 0.5)],
            vec![(0.0, -0.5), (1.0, 0.25), (2.0, 1.0)],
        ];

        let mut buffer = Samples::new(2, 20, vec![-1.0, -0.5, -0.25, 0.25, 0.5, 1.0]);
        View::<TestBackend>::process(&mut chart, &mut buffer).unwrap();

        assert_eq!(chart.axes, axes);
        assert_eq!(chart.points, expected);
    }
}
