use clap::Subcommand;

pub mod analyze;
pub mod find;
pub mod line_count;

use crate::commands::{find::Find, line_count::LineCount};

use self::analyze::Analyze;

#[derive(Subcommand, Debug)]
pub enum Command {
    LineCount(LineCount),
    Find(Find),
    Analyze(Analyze),
}

impl Command {
    pub fn eval(&self) {
        match self {
            Self::LineCount(cmd) => cmd.run(),
            Self::Analyze(cmd) => cmd.run(),
            Self::Find(cmd) => cmd.run(),
        }
    }
}
