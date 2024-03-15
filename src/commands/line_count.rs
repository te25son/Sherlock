use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;

use crate::contents::PathContents;

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
        let contents = PathContents::from(&self.path);

        let mut paths_by_directory = HashMap::new();

        for directory in contents.directories {
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
        let mut contents = PathContents::from(path.unwrap_or(&self.path));

        contents.add_filter(filter_paths_with_extension(self.file_extension.clone()));
        contents.add_filter(filter_out_paths_under_folders(
            self.excluded_folders.clone(),
        ));

        Ok(contents.get_files())
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

fn filter_out_paths_under_folders(folders: Vec<String>) -> impl Fn(&mut Vec<PathBuf>) -> () {
    move |paths| {
        paths.retain(|path| {
            !path
                .iter()
                .any(|c| folders.contains(&c.to_string_lossy().to_string()))
        })
    }
}

fn filter_paths_with_extension(extension: String) -> impl Fn(&mut Vec<PathBuf>) -> () {
    move |paths| {
        paths.retain(|path| {
            let path_extension = path.extension().and_then(|o| o.to_str());
            path_extension == Some(&extension)
        })
    }
}
