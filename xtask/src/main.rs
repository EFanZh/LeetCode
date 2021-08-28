use crate::coverage::Subcommand;
use std::env;
use structopt::StructOpt;

mod coverage;
mod tools;
mod utilities;

#[derive(StructOpt)]
enum Config {
    Coverage(Subcommand),
}

fn main() {
    assert_eq!(env::current_dir().unwrap(), utilities::get_project_dir());

    match Config::from_args() {
        Config::Coverage(command) => command.run(),
    }
}
