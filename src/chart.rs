use tui::symbols::Marker;
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType};

pub struct SignalChart<'a> {
    axes: (Axis<'a>, Axis<'a>),
    dataset: Dataset<'a>,
    points: Vec<Vec<(f64, f64)>>,
    title: String,
}

impl<'a> SignalChart<'a> {
    /// Create a new SignalChart.
    pub fn new(title: String, channels: usize, buffer_length: usize) -> Self {
        let length = buffer_length / channels;
        let x_axis = Axis::default().bounds([0.0f64, length as f64]);
        let y_axis = Axis::default().bounds([-1.0, 1.0]);

        let dataset = Dataset::default()
            .marker(Marker::Braille)
            .graph_type(GraphType::Line);

        let points = (0..length).map(|index| (index as f64, 0.0f64)).collect();

        SignalChart {
            axes: (x_axis, y_axis),
            dataset,
            points: vec![points; channels],
            title,
        }
    }

    /// Draw plots in terminal block.
    pub fn render(&self) -> Chart {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);

        let datasets = self
            .points
            .iter()
            .map(|points| self.dataset.clone().data(&points))
            .collect();

        Chart::new(datasets)
            .block(block)
            .x_axis(self.axes.0.clone())
            .y_axis(self.axes.1.clone())
    }

    /// Overwrite plot datasets from packer audio frame buffer.
    pub fn update(&mut self, buffer: &[f32]) {
        let channels = self.points.len();

        for points in self.points.iter_mut() {
            for (index, element) in points.iter_mut() {
                *element = buffer[channels * (*index as usize)] as f64;
            }
        }
    }
}
