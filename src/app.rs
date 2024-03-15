use crate::commands::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct App {
    #[command(subcommand)]
    pub command: Command,
}

impl App {
    pub fn run(&self) {
        self.command.eval();
    }
}
