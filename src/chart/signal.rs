use crate::action::{Action, CrossFrame};
use crate::buffer::SamplesBuffer;
use crate::chart::axes::Axes;
use crossterm::event::KeyEvent;
use tui::layout::Rect;
use tui::symbols::Marker;
use tui::widgets::{Block, Borders, Chart, Dataset, GraphType};

pub struct SignalChart<'a> {
    axes: Axes,
    dataset: Dataset<'a>,
    points: Vec<Vec<(f64, f64)>>,
    title: String,
}

impl<'a> SignalChart<'a> {
    /// Create a new SignalChart.
    pub fn new(title: String, channels: usize, frame_count: usize) -> Self {
        let axes = Axes::new([0.0f64, frame_count as f64], [-1.0, 1.0], 1.0);

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line);

        let points = (0..frame_count)
            .map(|index| (index as f64, 0.0f64))
            .collect();

        SignalChart {
            axes,
            dataset,
            points: vec![points; channels],
            title,
        }
    }
}

impl<'a> Action for SignalChart<'a> {
    fn key_event(&mut self, event: KeyEvent) {
        self.axes.key_event(event);
    }

    /// Overwrite plot datasets from packer audio frame buffer.
    fn process(&mut self, buffer: &mut SamplesBuffer) {
        let channels = buffer.channels as usize;
        let frame_count = buffer.data.len() / channels;

        let points = (0..frame_count)
            .map(|index| (index as f64, 0.0f64))
            .collect();
        self.points = vec![points; channels];

        for (outer_index, points) in self.points.iter_mut().enumerate() {
            for (inner_index, element) in points.iter_mut() {
                let index = channels * (*inner_index as usize) + outer_index;
                *element = buffer.data[index] as f64;
            }
        }
    }

    /// Draw plots in terminal block.
    fn render(&mut self, frame: &mut CrossFrame, area: Rect) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);

        let datasets = self
            .points
            .iter()
            .map(|points| self.dataset.clone().data(&points))
            .collect();

        let (x_axis, y_axis) = self.axes.axes();
        let chart = Chart::new(datasets)
            .block(block)
            .x_axis(x_axis)
            .y_axis(y_axis);

        frame.render_widget(chart, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_points() {
        let chart = SignalChart::new(String::from(""), 2, 3);
        let expected = vec![
            vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
            vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0)],
        ];

        assert_eq!(chart.points, expected);
    }

    #[test]
    fn process_points() {
        let mut chart = SignalChart::new(String::from(""), 1, 1);
        let expected = vec![
            vec![(0.0, -1.0), (1.0, -0.25), (2.0, 0.5)],
            vec![(0.0, -0.5), (1.0, 0.25), (2.0, 1.0)],
        ];

        let mut buffer = SamplesBuffer::new(2, 20, vec![-1.0, -0.5, -0.25, 0.25, 0.5, 1.0]);
        chart.process(&mut buffer);

        assert_eq!(chart.points, expected);
    }
}
