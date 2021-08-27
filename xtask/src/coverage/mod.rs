use crate::tools;
use serde_json::{Deserializer, Value};
use std::ffi::{OsStr, OsString};
use std::fmt::Write;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
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

    #[structopt(long)]
    llvm_version: Option<String>,

    #[structopt(long, default_value = "nightly")]
    rust_toolchain: String,

    #[structopt(short, long)]
    output_path: PathBuf,

    #[structopt(short = "t", long, default_value = "lcov")]
    output_type: OutputType,
}

fn run_coverage_tests(test_executable: &Path, llvm_profdata: &Path, llvm_cov: &Path, output: &Path) {
    let profile_dir = tempfile::tempdir().unwrap();
    let profraw_file = profile_dir.path().join("coverage.profraw");

    assert!(Command::new(test_executable)
        .env("LLVM_PROFILE_FILE", profraw_file.as_os_str())
        .status()
        .unwrap()
        .success());

    let profdata_file = profile_dir.path().join("coverage.profdata");

    assert!(Command::new(llvm_profdata)
        .args([
            "merge".as_ref(),
            "-o".as_ref(),
            profdata_file.as_os_str(),
            profraw_file.as_ref(),
        ])
        .status()
        .unwrap()
        .success());

    assert!(Command::new(llvm_cov)
        .args([
            "export".as_ref(),
            "--format".as_ref(),
            "lcov".as_ref(),
            "--instr-profile".as_ref(),
            profdata_file.as_os_str(),
            test_executable.as_os_str(),
        ])
        .stdout(OpenOptions::new().append(true).create(true).open(output).unwrap())
        .status()
        .unwrap()
        .success());
}

fn add_cmake_variable(command: &mut Command, variable: &str, value: &OsStr) {
    command.arg("-D");

    let mut arg = OsString::from_str(variable).unwrap();

    arg.push("=");
    arg.push(value);

    command.arg(arg);
}

fn run_cpp_tests(cmake_toolchain_file: Option<&Path>, llvm_version: Option<&str>, output: &Path) {
    const CPP_COVERAGE_TARGET_DIR: &str = "target/c++-coverage";

    let cmake_executable = tools::get_cmake().unwrap();
    let mut clang = String::from("clang");
    let mut clang_plus_plus = String::from("clang++");
    let mut llvm_profdata = String::from("llvm-profdata");
    let mut llvm_cov = String::from("llvm-cov");

    if let Some(llvm_version) = llvm_version {
        for tool in [&mut clang, &mut clang_plus_plus, &mut llvm_profdata, &mut llvm_cov] {
            write!(tool, "-{}", llvm_version).unwrap();
        }
    }

    // Configure.

    let mut cmake_command = Command::new(cmake_executable);

    cmake_command.args([
        "-S",
        "c++",
        "-B",
        CPP_COVERAGE_TARGET_DIR,
        "-G",
        "Ninja",
        "-D",
        "CMAKE_BUILD_TYPE=Debug",
        "-D",
        format!("CMAKE_C_COMPILER={}", clang).as_str(),
        "-D",
        format!("CMAKE_CXX_COMPILER={}", clang_plus_plus).as_str(),
        "-D",
        "ENABLE_SOURCE_BASED_CODE_COVERAGE=ON",
    ]);

    add_cmake_variable(
        &mut cmake_command,
        "CMAKE_MAKE_PROGRAM",
        tools::get_ninja().unwrap().as_os_str(),
    );

    if let Some(cmake_toolchain_file) = cmake_toolchain_file {
        add_cmake_variable(
            &mut cmake_command,
            "CMAKE_TOOLCHAIN_FILE",
            cmake_toolchain_file.as_os_str(),
        );
    }

    assert!(cmake_command.status().unwrap().success());

    // Build.

    let mut cmake_build_command = Command::new(cmake_executable);

    cmake_build_command.args(["--build", CPP_COVERAGE_TARGET_DIR, "-j"]);

    assert!(cmake_build_command.status().unwrap().success());

    // Run.

    #[cfg(target_os = "windows")]
    let executable = "leet-code-tests.exe";

    #[cfg(not(target_os = "windows"))]
    let executable = "leet-code-tests";

    run_coverage_tests(
        &Path::new(CPP_COVERAGE_TARGET_DIR).join(executable),
        llvm_profdata.as_ref(),
        llvm_cov.as_ref(),
        output,
    );
}

fn get_cargo_test_command(toolchain: &str) -> Command {
    let mut command = Command::new("cargo");

    command
        .args([format!("+{}", toolchain).as_str(), "test"])
        .env("RUSTFLAGS", "-Zinstrument-coverage");

    command
}

fn get_llvm_tools(toolchain: &str) -> (PathBuf, PathBuf) {
    let llvm_tool_dir = tools::find_rust_lib(toolchain);
    let mut llvm_profdata = llvm_tool_dir.clone();

    llvm_profdata.push("llvm-profdata");

    let mut llvm_cov = llvm_tool_dir;

    llvm_cov.push("llvm-cov");

    (llvm_profdata, llvm_cov)
}

fn run_rust_tests(toolchain: &str, output: &Path) {
    let mut build_process = get_cargo_test_command(toolchain)
        .args(["--no-run", "--message-format", "json"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let test_executable = Deserializer::from_reader(build_process.stdout.as_mut().unwrap())
        .into_iter::<Value>()
        .find_map(|item| {
            if let Ok(Value::Object(mut item)) = item {
                if let Some(Value::String(executable)) = item.remove("executable") {
                    Some(executable)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .unwrap();

    assert!(build_process.wait_with_output().unwrap().status.success());

    let (llvm_profdata, llvm_cov) = get_llvm_tools(toolchain);

    run_coverage_tests(test_executable.as_ref(), &llvm_profdata, &llvm_cov, output);
}

fn run_all_tests(cmake_toolchain_file: Option<&Path>, llvm_version: Option<&str>, rust_toolchain: &str, output: &Path) {
    run_cpp_tests(cmake_toolchain_file.as_deref(), llvm_version, output);
    run_rust_tests(rust_toolchain, output);
}

impl Subcommand {
    pub fn run(self) {
        let coverage_dir = tempfile::tempdir().unwrap();
        let coverage_file = coverage_dir.path().join("coverage.info");

        run_all_tests(
            self.cmake_toolchain_file.as_deref(),
            self.llvm_version.as_deref(),
            &self.rust_toolchain,
            &coverage_file,
        );

        if let Some(parent) = self.output_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        assert!(Command::new("grcov")
            .args([
                OsStr::new("--branch"),
                OsStr::new("--keep-only"),
                OsStr::new("c++/*"),
                OsStr::new("--keep-only"),
                OsStr::new("src/*"),
                OsStr::new("-o"),
                self.output_path.as_os_str(),
                OsStr::new("-t"),
                self.output_type.value().as_ref(),
                OsStr::new("-s"),
                OsStr::new("."),
                coverage_file.as_os_str()
            ])
            .status()
            .unwrap()
            .success());
    }
}
