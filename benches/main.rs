use criterion::{criterion_group, criterion_main, Criterion};
use sampitor::dsp::Samples;
use sampitor::io::path;
use sampitor::App;
use std::path::PathBuf;
use tempfile::NamedTempFile;
use tui::backend::TestBackend;
use tui::terminal::Terminal;

pub fn temp_wave_file(samples: &Samples) -> eyre::Result<PathBuf> {
    let path = NamedTempFile::new().unwrap().path().to_owned();
    path::write_samples(&path, samples)?;
    Ok(path)
}

pub fn init_benchmark(c: &mut Criterion) {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let mut app = App::new(&mut []);
    app.render(&mut terminal).unwrap();

    c.bench_function("render", |b| b.iter(|| app.render(&mut terminal).unwrap()));
}

criterion_group!(benches, init_benchmark);
criterion_main!(benches);
