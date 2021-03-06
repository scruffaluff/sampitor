//! File path navigators and parsers.

use color_eyre::eyre;
use std::cmp::Ordering;
use std::path::Path;

/// Get path file name or descriptive error.
///
/// # Errors
///
/// Will return `Err` if `path` contains an unparseable name.
pub fn name(path: &Path) -> eyre::Result<&str> {
    path.file_name()
        .ok_or_else(|| eyre::eyre!("File path {:?} does not have a final component", path))?
        .to_str()
        .ok_or_else(|| eyre::eyre!("File name {:?} is not valid Unicode", path))
}

/// Read inodes from a directory and sort them with subdirectories first
///
/// # Errors
///
/// Will return `Err` if `directory` does not exist or contains files whose metadata is unparseable.
pub fn sorted_names(directory: &Path) -> eyre::Result<Vec<(String, bool)>> {
    let mut files: Vec<(String, bool)> = vec![];

    for inode in directory.read_dir()? {
        let inode = inode?;
        files.push((
            name(&inode.path())?.to_string(),
            inode.file_type()?.is_dir(),
        ));
    }

    files.sort_by(|left, right| {
        if left.1 && !right.1 {
            Ordering::Less
        } else if !left.1 && right.1 {
            Ordering::Greater
        } else {
            left.0.cmp(&right.0)
        }
    });
    Ok(files)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::fs::{self, File};

    #[test]
    fn sort_folders_before_files() {
        let folder = tempfile::tempdir().unwrap().path().to_owned();
        fs::create_dir(&folder).unwrap();

        File::create(folder.join("a")).unwrap();
        fs::create_dir(folder.join("c")).unwrap();
        fs::create_dir(folder.join("b")).unwrap();
        File::create(folder.join("d")).unwrap();

        let expected = vec![
            (String::from("b"), true),
            (String::from("c"), true),
            (String::from("a"), false),
            (String::from("d"), false),
        ];
        let actual = sorted_names(&folder).unwrap();
        assert_eq!(actual, expected);
    }
}
