use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static VISUAL_STUDIO_PATH: OnceLock<Option<Box<Path>>> = OnceLock::new();

fn find_visual_studio() -> Option<&'static Path> {
    VISUAL_STUDIO_PATH
        .get_or_init(|| {
            let tool = cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "cl.exe")?;
            let mut path = tool.path();

            for _ in 0..8 {
                path = path.parent().unwrap();
            }

            Some(path.into())
        })
        .as_deref()
}

pub fn find_cmake() -> Option<PathBuf> {
    for path in [
        r"C:\Program Files\CMake\bin\cmake.exe",
        r"C:\Program Files (x86)\CMake\bin\cmake.exe",
    ] {
        if Path::is_file(path.as_ref()) {
            return Some(PathBuf::from(path));
        }
    }

    let cmake_path = find_visual_studio()?.join(r"Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe");

    cmake_path.is_file().then_some(cmake_path)
}

pub fn find_ninja() -> Option<PathBuf> {
    let ninja_path = find_visual_studio()?.join(r"Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe");

    ninja_path.is_file().then_some(ninja_path)
}
