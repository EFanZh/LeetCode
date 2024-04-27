use crate::utilities;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;

#[cfg(target_os = "windows")]
mod windows;

pub struct RustVersionMeta {
    pub host: String,
}

pub fn find_rust_version_meta() -> RustVersionMeta {
    let mut host = None;

    utilities::run_command_and_stream_output(Command::new("rustc").arg("-vV"), |stdout| {
        for line in BufReader::new(stdout).lines() {
            let line = line.unwrap();

            #[allow(clippy::option_if_let_else)] // False positive.
            if let Some(host_value) = line.strip_prefix("host: ") {
                host = Some(host_value.to_string());
            }
        }
    });

    RustVersionMeta { host: host.unwrap() }
}

pub fn find_rust_sysroot() -> PathBuf {
    let mut path_buffer = String::from_utf8(utilities::run_command_and_get_output(
        Command::new("rustc").args(["--print", "sysroot"]),
    ))
    .unwrap();

    path_buffer.pop();

    PathBuf::from(path_buffer)
}

fn which(name: &str) -> Option<PathBuf> {
    which::which(name).is_ok().then(|| name.into())
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub fn find_cmake() -> Option<PathBuf> {
            which("cmake").or_else(windows::find_cmake)
        }
    } else {
        pub fn find_cmake() -> Option<PathBuf> {
            which("cmake")
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        pub fn find_ninja() -> Option<PathBuf> {
            which("ninja").or_else(windows::find_ninja)
        }
    } else {
        pub fn find_ninja() -> Option<PathBuf> {
            which("ninja")
        }
    }
}
