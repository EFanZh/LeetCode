use crate::tools;
use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display, Formatter};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use std::{env, fs};
use structopt::StructOpt;

enum OutputType {
    Html,
    Lcov,
}

#[derive(Debug)]
struct ParseOutputTypeError;

impl Display for ParseOutputTypeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str("Unknown output type.")
    }
}

impl FromStr for OutputType {
    type Err = ParseOutputTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(Self::Html),
            "lcov" => Ok(Self::Lcov),
            _ => Err(ParseOutputTypeError),
        }
    }
}

impl Display for OutputType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(self.value())
    }
}

impl OutputType {
    fn value(&self) -> &'static str {
        match self {
            OutputType::Html => "html",
            OutputType::Lcov => "lcov",
        }
    }
}

#[derive(StructOpt)]
pub struct Subcommand {
    #[structopt(long)]
    cmake_toolchain_file: Option<PathBuf>,

    #[structopt(short, long)]
    output_path: PathBuf,

    #[structopt(short = "t", long, default_value = "lcov")]
    output_type: OutputType,
}

fn add_cmake_variable(command: &mut Command, variable: &str, value: &OsStr) {
    command.arg("-D");

    let mut arg = OsString::from_str(variable).unwrap();

    arg.push("=");
    arg.push(value);

    command.arg(arg);
}

impl Subcommand {
    pub fn run(self) {
        let cmake_executable = tools::get_cmake().unwrap();

        // Configure C++.

        let mut cmake_command = Command::new(cmake_executable);

        cmake_command.args(&[
            "-S",
            "c++",
            "-B",
            "target/c++",
            "-G",
            "Ninja",
            "-D",
            "CMAKE_BUILD_TYPE=Debug",
            "-D",
            "CMAKE_C_COMPILER=clang",
            "-D",
            "CMAKE_CXX_COMPILER=clang++",
            "-D",
            "ENABLE_SOURCE_BASED_CODE_COVERAGE=ON",
        ]);

        add_cmake_variable(
            &mut cmake_command,
            "CMAKE_MAKE_PROGRAM",
            tools::get_ninja().unwrap().as_os_str(),
        );

        if let Some(cmake_toolchain_file) = self.cmake_toolchain_file {
            add_cmake_variable(
                &mut cmake_command,
                "CMAKE_TOOLCHAIN_FILE",
                cmake_toolchain_file.as_os_str(),
            );
        }

        assert!(cmake_command.status().unwrap().success());

        // Build C++.

        let mut cmake_build_command = Command::new(cmake_executable);

        cmake_build_command.args(&["--build", "target/c++", "-j"]);

        // Workaround for https://github.com/microsoft/vcpkg/issues/11467.

        if cfg!(target_os = "windows") {
            let mut paths = tools::get_msvc_binary_tools_dir().unwrap().as_os_str().to_os_string();

            paths.push(";");
            paths.push(env::var_os("Path").unwrap());

            cmake_build_command.env("Path", paths);
        }

        assert!(cmake_build_command.status().unwrap().success());

        // Run C++.

        let profile_dir = tempfile::tempdir().unwrap();

        assert!(Command::new("target/c++/leet-code-tests")
            .env("LLVM_PROFILE_FILE", profile_dir.path().join("c++.profraw"))
            .status()
            .unwrap()
            .success());

        // Run Rust.

        assert!(Command::new("cargo")
            .args(&["+nightly", "test"])
            .envs(
                [
                    ("LLVM_PROFILE_FILE", profile_dir.path().join("rust.profraw").as_os_str()),
                    ("RUSTFLAGS", OsStr::new("-Zinstrument-coverage"))
                ]
                .iter()
                .copied()
            )
            .status()
            .unwrap()
            .success());

        // Generate report with grcov.

        if let Some(parent) = self.output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        assert!(Command::new("grcov")
            .args(&[
                OsStr::new("--branch"),
                OsStr::new("-b"),
                OsStr::new("target"),
                OsStr::new("--keep-only"),
                OsStr::new("c++/*"),
                OsStr::new("--keep-only"),
                OsStr::new("src/*"),
                OsStr::new("-o"),
                self.output_path.as_os_str(),
                OsStr::new("-t"),
                OsStr::new(self.output_type.value()),
                OsStr::new("-s"),
                OsStr::new("."),
                profile_dir.path().as_os_str()
            ])
            .env("RUSTUP_TOOLCHAIN", "nightly")
            .status()
            .unwrap()
            .success());
    }
}
