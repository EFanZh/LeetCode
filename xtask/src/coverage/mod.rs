use crate::{tools, utilities};
use serde_json::{Deserializer, Value};
use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display, Formatter, Write};
use std::fs::{self, File};
use std::path::{self, Path, PathBuf};
use std::process::Command;
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

fn run_coverage_tests(test_executable: &Path, llvm_profdata: &Path, output: &Path) {
    let profile_dir = tempfile::tempdir().unwrap();
    let profraw_file = profile_dir.path().join("coverage.profraw");

    utilities::run_command(Command::new(test_executable).env("LLVM_PROFILE_FILE", profraw_file.as_os_str()));

    utilities::run_command(Command::new(llvm_profdata).args([
        "merge".as_ref(),
        "-o".as_ref(),
        output.as_os_str(),
        profraw_file.as_ref(),
    ]));
}

fn add_cmake_variable(command: &mut Command, variable: &str, value: &OsStr) {
    command.arg("-D");

    let mut arg = OsString::from_str(variable).unwrap();

    arg.push("=");
    arg.push(value);

    command.arg(arg);
}

fn run_cpp_tests(cmake_toolchain_file: Option<&Path>, llvm_version: Option<&str>, output: &Path) -> PathBuf {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            const CPP_COVERAGE_TARGET_DIR: &str = "target\\c++-coverage";
        } else {
            const CPP_COVERAGE_TARGET_DIR: &str = "target/c++-coverage";
        }
    }

    let cmake_executable = tools::get_cmake().unwrap();
    let mut clang = String::from("clang");
    let mut clang_plus_plus = String::from("clang++");
    let mut llvm_profdata = String::from("llvm-profdata");

    if let Some(llvm_version) = llvm_version {
        for tool in [&mut clang, &mut clang_plus_plus, &mut llvm_profdata] {
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

    utilities::run_command(&mut cmake_command);

    // Build.

    utilities::run_command(Command::new(cmake_executable).args(["--build", CPP_COVERAGE_TARGET_DIR, "-j"]));

    // Run.

    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let executable_name = "leet-code-tests.exe";
        } else {
            let executable_name = "leet-code-tests";
        }
    }

    let test_executable = Path::new(CPP_COVERAGE_TARGET_DIR).join(executable_name);

    run_coverage_tests(&test_executable, llvm_profdata.as_ref(), output);

    test_executable
}

fn run_rust_tests(toolchain: &str, llvm_profdata: &Path, output: &Path) -> PathBuf {
    // Build.

    let test_executable = utilities::run_command_and_stream_output(
        Command::new("cargo")
            .args([
                format!("+{}", toolchain).as_str(),
                "test",
                "--no-run",
                "--message-format",
                "json",
            ])
            .env("RUSTFLAGS", "-Zinstrument-coverage"),
        |stdout| {
            Deserializer::from_reader(stdout).into_iter::<Value>().find_map(|item| {
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
        },
    )
    .unwrap();

    // Run.

    run_coverage_tests(test_executable.as_ref(), llvm_profdata, output);

    PathBuf::from(test_executable)
}

fn get_llvm_tools(toolchain: &str) -> (PathBuf, PathBuf) {
    let llvm_tool_dir = tools::find_rust_lib(toolchain);
    let mut llvm_profdata = llvm_tool_dir.clone();

    llvm_profdata.push("llvm-profdata");

    let mut llvm_cov = llvm_tool_dir;

    llvm_cov.push("llvm-cov");

    (llvm_profdata, llvm_cov)
}

impl Subcommand {
    pub fn run(self) {
        let coverage_dir = tempfile::tempdir().unwrap();
        let cpp_profdata = coverage_dir.path().join("c++.profdata");
        let rust_profdata = coverage_dir.path().join("rust.profdata");
        let all_profdata = coverage_dir.path().join("all.profdata");
        let (llvm_profdata, llvm_cov) = get_llvm_tools(&self.rust_toolchain);

        // Run tests.

        let cpp_test_executable = run_cpp_tests(
            self.cmake_toolchain_file.as_deref(),
            self.llvm_version.as_deref(),
            &cpp_profdata,
        );

        let rust_test_executable = run_rust_tests(&self.rust_toolchain, &llvm_profdata, &rust_profdata);

        // Merge profile data.

        utilities::run_command(Command::new(llvm_profdata).args([
            "merge".as_ref(),
            "-o".as_ref(),
            all_profdata.as_os_str(),
            cpp_profdata.as_ref(),
            rust_profdata.as_ref(),
        ]));

        // Generate report.

        let mut path_equivalence = OsString::from("src,");

        path_equivalence.push(utilities::get_project_dir());
        path_equivalence.push(path::MAIN_SEPARATOR.encode_utf8(&mut [0]));
        path_equivalence.push("src");

        match self.output_type {
            OutputType::Html => {
                utilities::run_command(Command::new(llvm_cov).args([
                    "show".as_ref(),
                    "--format".as_ref(),
                    "html".as_ref(),
                    "--output-dir".as_ref(),
                    self.output_path.as_os_str(),
                    "--Xdemangler".as_ref(),
                    "--path-equivalence".as_ref(),
                    path_equivalence.as_os_str(),
                    "rustfilt".as_ref(),
                    "--instr-profile".as_ref(),
                    all_profdata.as_os_str(),
                    cpp_test_executable.as_os_str(),
                    "--object".as_ref(),
                    rust_test_executable.as_os_str(),
                    // "c++".as_ref(),
                    // "src".as_ref(),
                ]));
            }
            OutputType::Lcov => {
                if let Some(parent_dir) = self.output_path.parent() {
                    fs::create_dir_all(parent_dir).unwrap();
                }

                utilities::run_command_and_redirect_output(
                    Command::new(llvm_cov).args([
                        "export".as_ref(),
                        "--format".as_ref(),
                        "lcov".as_ref(),
                        "--path-equivalence".as_ref(),
                        path_equivalence.as_os_str(),
                        "--instr-profile".as_ref(),
                        all_profdata.as_os_str(),
                        cpp_test_executable.as_os_str(),
                        "--object".as_ref(),
                        rust_test_executable.as_os_str(),
                        "c++".as_ref(),
                        "src".as_ref(),
                    ]),
                    File::create(self.output_path).unwrap(),
                );
            }
        }
    }
}
