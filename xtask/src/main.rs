use crate::coverage::Subcommand;
use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

mod coverage;
mod tools;

#[derive(StructOpt)]
enum Config {
    Coverage(Subcommand),
}

fn get_project_root() -> PathBuf {
    let mut path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    path.pop();

    path
}

fn main() {
    assert_eq!(env::current_dir().unwrap(), get_project_root());

    match Config::from_args() {
        Config::Coverage(command) => command.run(),
    }
}
