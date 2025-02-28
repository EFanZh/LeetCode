#![expect(missing_docs, reason = "unnecessary")]

use crate::coverage::Subcommand;
use clap::Parser;
use std::env;

mod coverage;
mod tools;
mod utilities;

#[derive(Parser)]
enum Config {
    Coverage(Subcommand),
}

fn main() {
    assert_eq!(env::current_dir().unwrap(), utilities::get_project_dir());

    match Config::parse() {
        Config::Coverage(command) => command.run(),
    }
}
