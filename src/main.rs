mod app;
mod commands;
mod contents;
mod utils;

use crate::app::App;

use clap::Parser;

fn main() {
    App::parse().run();
}
