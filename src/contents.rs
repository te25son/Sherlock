use std::path::PathBuf;

use crate::utils::get_ok_entries_from_path;

type Filter = Box<dyn Fn(&mut Vec<PathBuf>) -> ()>;

#[derive(Default)]
pub struct PathContents {
    pub directories: Vec<PathBuf>,

    files: Vec<PathBuf>,
    filters: Vec<Filter>,
}

impl PathContents {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_files(&mut self) -> Vec<PathBuf> {
        self.walk().files.clone()
    }

    pub fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&mut Vec<PathBuf>) -> () + 'static,
    {
        self.filters.push(Box::new(filter));
    }

    fn walk(&mut self) -> &mut Self {
        self.directories
            .iter()
            .for_each(|d| self.files.append(&mut Self::from(d).walk().files));
        self.apply_filters();
        self
    }

    fn apply_filters(&mut self) {
        self.filters
            .iter()
            .for_each(|filter| filter(&mut self.files))
    }
}

impl From<&PathBuf> for PathContents {
    fn from(path: &PathBuf) -> Self {
        let mut path_items = Self::new();
        let valid_paths = get_ok_entries_from_path(path).into_iter().map(|e| e.path());

        for path in valid_paths {
            if path.is_dir() {
                path_items.directories.push(path)
            } else if path.is_file() {
                path_items.files.push(path)
            }
        }
        path_items
    }
}
