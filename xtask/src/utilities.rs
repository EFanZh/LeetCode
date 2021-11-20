use std::env;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn get_project_dir() -> PathBuf {
    let mut path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    path.pop();

    path
}

#[allow(clippy::print_stdout, clippy::use_debug)]
fn print_command_line(command: &Command) {
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    writeln!(stdout, "===> {:?}", command).unwrap();
    stdout.flush().unwrap();
}

pub fn run_command(command: &mut Command) {
    print_command_line(command);

    let status = command.status().unwrap();

    assert!(status.success());
}

pub fn run_command_and_get_output(command: &mut Command) -> Vec<u8> {
    print_command_line(command);

    let output = command.output().unwrap();

    assert!(output.status.success());

    output.stdout
}

pub fn run_command_and_stream_output<R>(command: &mut Command, callback: impl FnOnce(&mut dyn Read) -> R) -> R {
    command.stdout(Stdio::piped());

    print_command_line(command);

    let mut child = command.spawn().unwrap();
    let result = callback(child.stdout.as_mut().unwrap());

    assert!(child.wait().unwrap().success());

    result
}

pub fn run_command_and_redirect_output(command: &mut Command, output: impl Into<Stdio>) {
    command.stdout(output);

    print_command_line(command);

    let status = command.status().unwrap();

    assert!(status.success());
}
