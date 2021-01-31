use std::path::Path;
use std::path::PathBuf;

lazy_static::lazy_static! {
    static ref MSVC_BINARY_TOOLS_DIR: Option<PathBuf> = find_msvc_binary_tools_dir();
    static ref VISUAL_STUDIO_PATH: Option<PathBuf> = find_visual_studio();
    static ref CMAKE_PATH: Option<PathBuf> = find_cmake();
    static ref NINJA_PATH: Option<PathBuf> = find_ninja();
}

fn find_msvc_binary_tools_dir() -> Option<PathBuf> {
    cc::windows_registry::find_tool("x86_64-pc-windows-msvc", "cl.exe").map(|tool| {
        let mut path = tool.path().to_path_buf();

        path.pop();

        path
    })
}

pub fn get_msvc_binary_tools_dir() -> Option<&'static Path> {
    MSVC_BINARY_TOOLS_DIR.as_deref()
}

fn find_visual_studio() -> Option<PathBuf> {
    MSVC_BINARY_TOOLS_DIR.as_deref().map(|path| {
        let mut path = path.to_path_buf();

        for _ in 0..7 {
            path.pop();
        }

        path
    })
}

fn find_cmake() -> Option<PathBuf> {
    if which::which("cmake").is_ok() {
        Some(PathBuf::from("cmake"))
    } else if cfg!(target_os = "windows") {
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
}

pub fn get_cmake() -> Option<&'static Path> {
    CMAKE_PATH.as_deref()
}

fn find_ninja() -> Option<PathBuf> {
    if which::which("ninja").is_ok() {
        Some(PathBuf::from("ninja"))
    } else if cfg!(target_os = "windows") {
        VISUAL_STUDIO_PATH
            .as_deref()
            .map(|visual_studio_path| {
                visual_studio_path.join(r"Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe")
            })
            .filter(|path| path.is_file())
    } else {
        None
    }
}

pub fn get_ninja() -> Option<&'static Path> {
    NINJA_PATH.as_deref()
}
