#[cfg(test)]
pub mod test {
    use crate::dsp::Samples;
    use crate::io::path;
    use std::fmt::Write;
    use std::fs;
    use std::path::PathBuf;
    use tui::buffer::Buffer;
    use unicode_width::UnicodeWidthStr;
    use uuid::Uuid;

    /// Returns a string representation of the given buffer for debugging purpose.
    ///
    /// Copied from https://github.com/fdehau/tui-rs/blob/master/src/backend/test.rs for easier testing.
    pub fn buffer_view(buffer: &Buffer) -> String {
        let mut view =
            String::with_capacity(buffer.content.len() + buffer.area.height as usize * 3);
        for cells in buffer.content.chunks(buffer.area.width as usize) {
            let mut overwritten = vec![];
            let mut skip: usize = 0;
            view.push('"');
            for (x, c) in cells.iter().enumerate() {
                if skip == 0 {
                    view.push_str(&c.symbol);
                } else {
                    overwritten.push((x, &c.symbol))
                }
                skip = std::cmp::max(skip, c.symbol.width()).saturating_sub(1);
            }
            view.push('"');
            if !overwritten.is_empty() {
                write!(
                    &mut view,
                    " Hidden by multi-width symbols: {:?}",
                    overwritten
                )
                .unwrap();
            }
            view.push('\n');
        }
        view
    }

    pub fn temp_wave_file(samples: &Samples) -> eyre::Result<PathBuf> {
        let folder = tempfile::tempdir()?.path().to_owned();
        fs::create_dir_all(&folder)?;
        let path = folder.join(format!("{}.txt", Uuid::new_v4()));

        path::write_samples(&path, samples)?;
        Ok(path)
    }
}
