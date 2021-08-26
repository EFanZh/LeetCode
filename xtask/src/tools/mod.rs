use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "windows")]
mod windows;

lazy_static::lazy_static! {
    static ref CMAKE_PATH: Option<PathBuf> = find_cmake();
    static ref NINJA_PATH: Option<PathBuf> = find_ninja();
}

fn find_cmake() -> Option<PathBuf> {
    let result = which::which("cmake").ok();

    #[cfg(target_os = "windows")]
    let result = result.or_else(windows::find_cmake);

    result
}

pub fn get_cmake() -> Option<&'static Path> {
    CMAKE_PATH.as_deref()
}

fn find_ninja() -> Option<PathBuf> {
    let result = which::which("ninja").ok();

    #[cfg(target_os = "windows")]
    let result = result.or_else(windows::find_ninja);

    result
}

pub fn get_ninja() -> Option<&'static Path> {
    NINJA_PATH.as_deref()
}
