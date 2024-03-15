use clap::Parser;

#[derive(Debug, Parser)]
pub struct Find {
    pattern: String,
}

impl Find {
    pub fn run(&self) {
        todo!()
    }
}
