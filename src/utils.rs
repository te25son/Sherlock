use std::{fs::read_dir, path::PathBuf};

pub fn get_ok_entry_paths(path: &PathBuf) -> Vec<PathBuf> {
    match read_dir(path) {
        Ok(dir) => dir.filter_map(|d| d.ok()).map(|e| e.path()).collect(),
        _ => vec![],
    }
}
