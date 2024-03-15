mod app;
mod commands;
mod content;
mod utils;

use crate::app::App;

use clap::Parser;

fn main() {
    App::parse().run();
}
