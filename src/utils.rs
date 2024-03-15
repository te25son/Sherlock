use std::{
    fs::{read_dir, DirEntry},
    path::PathBuf,
};

pub fn get_ok_entries_from_path(path: &PathBuf) -> Vec<DirEntry> {
    match read_dir(path) {
        Ok(dir) => dir.filter_map(|d| d.ok()).collect(),
        _ => vec![],
    }
}
