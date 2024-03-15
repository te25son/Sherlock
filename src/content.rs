use std::path::PathBuf;

use crate::utils::get_ok_entry_paths;

type Filter = Box<dyn Fn(&PathBuf) -> bool>;

#[derive(Default)]
pub struct Content {
    pub files: Vec<PathBuf>,

    filters: Vec<Filter>,
}

impl Content {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn add_filter<F>(&mut self, filter: F) -> &mut Self
    where
        F: Fn(&PathBuf) -> bool + 'static,
    {
        self.filters.push(Box::new(filter));
        self
    }

    pub fn explore_path(&mut self, path: &PathBuf) {
        for path in get_ok_entry_paths(&path) {
            if path.is_file() && self.filters.iter().all(|f| f(&path)) {
                self.files.push(path)
            } else if path.is_dir() {
                self.explore_path(&path)
            }
        }
    }
}
