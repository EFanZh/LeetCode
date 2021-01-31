use std::path::Path;
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref MSVC_BINARY_TOOLS_DIR: Option<PathBuf> = find_msvc_binary_tools_dir();
    static ref VISUAL_STUDIO_PATH: Option<PathBuf> = find_visual_studio();
    static ref CMAKE_PATH: Option<PathBuf> = find_cmake();
    static ref NINJA_PATH: Option<PathBuf> = find_ninja();
}

fn find_msvc_binary_tools_dir() -> Option<PathBuf> {
    cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "cl.exe")
        .map(|tool| tool.path().parent().unwrap().to_path_buf())
}

pub fn get_msvc_binary_tools_dir() -> Option<&'static Path> {
    MSVC_BINARY_TOOLS_DIR.as_deref()
}

fn find_visual_studio() -> Option<PathBuf> {
    MSVC_BINARY_TOOLS_DIR.as_deref().map(|mut path| {
        for _ in 0..7 {
            path = path.parent().unwrap();
        }

        path.to_path_buf()
    })
}

fn find_cmake() -> Option<PathBuf> {
    which::which("cmake").ok().or_else(|| {
        if cfg!(target_os = "windows") {
            for &path in &[
                r"C:\Program Files\CMake\bin\cmake.exe",
                r"C:\Program Files (x86)\CMake\bin\cmake.exe",
            ] {
                if Path::new(path).is_file() {
                    return Some(PathBuf::from(path));
                }
            }

            VISUAL_STUDIO_PATH
                .as_deref()
                .map(|visual_studio_path| {
                    visual_studio_path.join(r"Common7\IDE\CommonExtensions\Microsoft\CMake\CMake\bin\cmake.exe")
                })
                .filter(|path| path.is_file())
        } else {
            None
        }
    })
}

pub fn get_cmake() -> Option<&'static Path> {
    CMAKE_PATH.as_deref()
}

fn find_ninja() -> Option<PathBuf> {
    which::which("ninja").ok().or_else(|| {
        if cfg!(target_os = "windows") {
            VISUAL_STUDIO_PATH
                .as_deref()
                .map(|visual_studio_path| {
                    visual_studio_path.join(r"Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe")
                })
                .filter(|path| path.is_file())
        } else {
            None
        }
    })
}

pub fn get_ninja() -> Option<&'static Path> {
    NINJA_PATH.as_deref()
}
