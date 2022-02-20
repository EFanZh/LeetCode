use std::path::{Path, PathBuf};

lazy_static::lazy_static! {
    static ref VISUAL_STUDIO_PATH: Option<PathBuf> = find_visual_studio();
}

fn find_visual_studio() -> Option<PathBuf> {
    let mut buffer = cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "cl.exe")?
        .path()
        .to_path_buf();

    for _ in 0..8 {
        buffer.pop();
    }

    Some(buffer)
}

pub fn find_cmake() -> Option<PathBuf> {
    for path in [
        r"C:\Program Files\CMake\bin\cmake.exe",
        r"C:\Program Files (x86)\CMake\bin\cmake.exe",
    ] {
        if Path::new(path).is_file() {
            return Some(PathBuf::from(path));
        }
    }

    let cmake_path = VISUAL_STUDIO_PATH
        .as_deref()?
        .join(r"Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe");

    cmake_path.is_file().then(|| cmake_path)
}

pub fn find_ninja() -> Option<PathBuf> {
    let ninja_path = VISUAL_STUDIO_PATH
        .as_deref()?
        .join(r"Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe");

    ninja_path.is_file().then(|| ninja_path)
}
