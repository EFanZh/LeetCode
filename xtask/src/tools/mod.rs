use crate::utilities;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "windows")]
mod windows;

lazy_static::lazy_static! {
    static ref CMAKE_PATH: Option<PathBuf> = find_cmake();
    static ref NINJA_PATH: Option<PathBuf> = find_ninja();
}

fn find_rustc_host(toolchain: &str) -> String {
    utilities::run_command_and_stream_output(
        Command::new("rustc").args([format!("+{}", toolchain).as_str(), "-vV"]),
        |stdout| {
            BufReader::new(stdout)
                .lines()
                .find_map(|line| line.unwrap().strip_prefix("host: ").map(str::to_string))
        },
    )
    .unwrap()
}

pub fn find_rust_lib(toolchain: &str) -> PathBuf {
    let mut path_buffer = String::from_utf8(utilities::run_command_and_get_output(Command::new("rustc").args([
        format!("+{}", toolchain).as_str(),
        "--print",
        "sysroot",
    ])))
    .unwrap();

    path_buffer.pop();

    let mut path = PathBuf::from(path_buffer);

    path.extend(["lib", "rustlib", find_rustc_host(toolchain).as_str(), "bin"]);

    path
}

fn find_cmake_common() -> Option<PathBuf> {
    which::which("cmake").ok()
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        fn find_cmake() -> Option<PathBuf> {
            find_cmake_common().or_else(windows::find_cmake)
        }
    } else {
        fn find_cmake() -> Option<PathBuf> {
            find_cmake_common()
        }
    }
}

pub fn get_cmake() -> Option<&'static Path> {
    CMAKE_PATH.as_deref()
}

fn find_ninja_common() -> Option<PathBuf> {
    which::which("ninja").ok()
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        fn find_ninja() -> Option<PathBuf> {
            find_ninja_common().or_else(windows::find_ninja)
        }
    } else {
        fn find_ninja() -> Option<PathBuf> {
            find_ninja_common()
        }
    }
}

pub fn get_ninja() -> Option<&'static Path> {
    NINJA_PATH.as_deref()
}
