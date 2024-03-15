use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

use crate::{content::Content, utils::get_ok_entry_paths};

#[derive(Debug, Parser)]
pub struct LineCount {
    path: PathBuf,

    #[arg(
        short = 'x',
        long,
        help = "File extension to search for, e.g. 'py', 'rs', 'js', etc..."
    )]
    file_extension: String,

    #[arg(short, long, help = "Number of files to display. Defaults to 10.")]
    top: Option<usize>,

    #[arg(short = 'f', long, help = "Specific folders to ignore.")]
    excluded_folders: Vec<String>,

    #[arg(
        long,
        help = "Whether to group the line count by files in the given folder. Defaults to false."
    )]
    grouped: bool,
}

impl LineCount {
    pub fn run(&self) {
        match self.grouped {
            true => self.display_lines_by_directory(),
            false => self.display_lines_by_path(),
        }
    }

    fn display_lines_by_path(&self) {
        if let Ok(file_paths) = self.get_files(None) {
            let mut entries = get_file_length(file_paths);
            self.display(&mut entries);
        }
    }

    fn display_lines_by_directory(&self) {
        if let Ok(lines_by_directory) = self.get_total_lines_per_directory() {
            let mut entries = lines_by_directory.into_iter().collect::<Vec<_>>();
            self.display(&mut entries);
        }
    }

    fn display(&self, entries: &mut Vec<(PathBuf, usize)>) {
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        for entry in entries.iter().take(self.top.unwrap_or(10)) {
            println!("{}, {:?}", entry.1, entry.0)
        }
    }

    fn get_total_lines_per_directory(&self) -> Result<HashMap<PathBuf, usize>> {
        let mut directories = vec![];

        for path in get_ok_entry_paths(&self.path) {
            if path.is_dir() {
                directories.push(path)
            }
        }

        let mut paths_by_directory = HashMap::new();

        for directory in directories {
            if let Ok(file_paths) = self.get_files(Some(&directory)) {
                let total_lines = get_file_length(file_paths)
                    .iter()
                    .map(|p| p.1)
                    .sum::<usize>();
                paths_by_directory.insert(directory, total_lines);
            }
        }

        Ok(paths_by_directory)
    }

    fn get_files(&self, path: Option<&PathBuf>) -> Result<Vec<PathBuf>> {
        let mut content = Content::new();

        content
            .add_filter(extension_equals(self.file_extension.clone()))
            .add_filter(file_not_under_folders(self.excluded_folders.clone()))
            .explore_path(path.unwrap_or(&self.path));

        Ok(content.files)
    }
}

fn get_file_length(file_paths: Vec<PathBuf>) -> Vec<(PathBuf, usize)> {
    let mut file_length_by_file = vec![];
    for path in file_paths {
        if let Ok(file) = File::open(&path) {
            let reader = BufReader::new(file);
            file_length_by_file.push((path, reader.lines().count()))
        }
    }
    file_length_by_file
}

fn extension_equals(extension: String) -> impl Fn(&PathBuf) -> bool {
    move |p| p.extension() == Some(OsStr::new(&extension))
}

fn file_not_under_folders(folders: Vec<String>) -> impl Fn(&PathBuf) -> bool {
    move |p| {
        !p.iter()
            .any(|c| folders.contains(&c.to_string_lossy().to_string()))
    }
}
