use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Analyze {
    path: PathBuf,
}

impl Analyze {
    pub fn run(&self) {
        todo!()
    }
}
