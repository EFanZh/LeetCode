use crate::{tools, utilities};
use clap::Parser;
use serde_json::{Deserializer, Value};
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fmt::{self, Display, Formatter, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::{env, io};

#[derive(Clone)]
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

impl Error for ParseOutputTypeError {}

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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(self.value())
    }
}

impl OutputType {
    const fn value(&self) -> &'static str {
        match self {
            Self::Html => "html",
            Self::Lcov => "lcov",
        }
    }
}

#[derive(Parser)]
pub struct Subcommand {
    #[clap(long)]
    cmake_toolchain_file: Option<PathBuf>,
    #[clap(long)]
    llvm_version: Option<String>,
    #[clap(short, long)]
    output_path: PathBuf,
    #[clap(short = 't', long, default_value = "lcov")]
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
    let cpp_coverage_target_dir = Path::new("target").join(format!("c++-coverage-{}", env::consts::OS));
    let cmake_executable = tools::find_cmake().expect("Unable to find cmake executable.");
    let mut clang = String::from("clang");
    let mut clang_plus_plus = String::from("clang++");
    let mut llvm_profdata = String::from("llvm-profdata");

    if let Some(llvm_version) = llvm_version {
        for tool in [&mut clang, &mut clang_plus_plus, &mut llvm_profdata] {
            write!(tool, "-{llvm_version}").unwrap();
        }
    }

    // Configure.

    let mut cmake_command = Command::new(&cmake_executable);

    cmake_command.args([
        "-S".as_ref(),
        "c++".as_ref(),
        "-B".as_ref(),
        cpp_coverage_target_dir.as_os_str(),
        "-G".as_ref(),
        "Ninja".as_ref(),
        "-D".as_ref(),
        "CMAKE_BUILD_TYPE=Debug".as_ref(),
        "-D".as_ref(),
        format!("CMAKE_C_COMPILER={clang}").as_ref(),
        "-D".as_ref(),
        format!("CMAKE_CXX_COMPILER={clang_plus_plus}").as_ref(),
        "-D".as_ref(),
        "ENABLE_SOURCE_BASED_CODE_COVERAGE=ON".as_ref(),
    ]);

    add_cmake_variable(
        &mut cmake_command,
        "CMAKE_MAKE_PROGRAM",
        tools::find_ninja()
            .expect("Unable to find ninja executable.")
            .as_os_str(),
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

    utilities::run_command(Command::new(cmake_executable).args([
        "--build".as_ref(),
        cpp_coverage_target_dir.as_os_str(),
        "-j".as_ref(),
    ]));

    // Run.

    cfg_if::cfg_if! {
        if #[cfg(target_os = "windows")] {
            let executable_name = "leet-code-tests.exe";
        } else {
            let executable_name = "leet-code-tests";
        }
    }

    let test_executable = cpp_coverage_target_dir.join(executable_name);

    run_coverage_tests(&test_executable, llvm_profdata.as_ref(), output);

    test_executable
}

fn run_rust_tests(target_dir: &Path, llvm_profdata: &Path, output: &Path) -> PathBuf {
    // Build.

    let test_executable = utilities::run_command_and_stream_output(
        Command::new("cargo").args::<_, &OsStr>([
            "rustc".as_ref(),
            "--tests".as_ref(),
            "--target-dir".as_ref(),
            target_dir.as_ref(),
            "--message-format".as_ref(),
            "json".as_ref(),
            "--".as_ref(),
            "-C".as_ref(),
            "instrument-coverage".as_ref(),
        ]),
        |child_stdout| {
            let stdout = io::stdout();
            let mut stdout = stdout.lock();

            Deserializer::from_reader(child_stdout)
                .into_iter::<Value>()
                .find_map(|item| {
                    if let Ok(Value::Object(mut item)) = item {
                        if let Some(Value::Object(mut message)) = item.remove("message") {
                            if let Some(Value::String(rendered)) = message.remove("rendered") {
                                use std::io::Write;

                                stdout.write_all(rendered.as_bytes()).unwrap();
                                stdout.flush().unwrap();
                            }
                        }

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

const fn closure_type_deduction_helper<T>(f: T) -> T
where
    T: for<'a> FnOnce(&'a mut Command) -> &'a mut Command,
{
    f
}

impl Subcommand {
    pub fn run(self) {
        let rust_version_meta = tools::find_rust_version_meta();

        let rust_lib_path = {
            let mut buffer = tools::find_rust_sysroot();

            buffer.extend(["lib", "rustlib"]);

            buffer
        };

        let coverage_dir = tempfile::tempdir().unwrap();
        let cpp_profdata = coverage_dir.path().join("c++.profdata");
        let rust_profdata = coverage_dir.path().join("rust.profdata");
        let all_profdata = coverage_dir.path().join("all.profdata");

        let (llvm_profdata, llvm_cov) = {
            let mut buffer = rust_lib_path.join(rust_version_meta.host.as_str());

            buffer.push("bin");

            let llvm_profdata = buffer.join("llvm-profdata");

            buffer.push("llvm-cov");

            (llvm_profdata, buffer)
        };

        // Run tests.

        let cpp_test_executable = run_cpp_tests(
            self.cmake_toolchain_file.as_deref(),
            self.llvm_version.as_deref(),
            &cpp_profdata,
        );

        let mut rust_target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        rust_target_dir.pop();
        rust_target_dir.push("target");
        rust_target_dir.push("rust-coverage");

        let rust_test_executable = run_rust_tests(&rust_target_dir, &llvm_profdata, &rust_profdata);

        // Merge profile data.

        utilities::run_command(Command::new(llvm_profdata).args([
            "merge".as_ref(),
            "-o".as_ref(),
            all_profdata.as_os_str(),
            cpp_profdata.as_ref(),
            rust_profdata.as_ref(),
        ]));

        // Generate report.

        let add_common_llvm_cov_args = closure_type_deduction_helper(|command: &mut Command| {
            const SEPARATOR_REGEX: &str = if cfg!(windows) { r"\\" } else { "/" };

            command.args([
                "--ignore-filename-regex".as_ref(),
                format!(
                    r"{SEPARATOR_REGEX}rustc{SEPARATOR_REGEX}([0-9a-f]+|[0-9]+\.[0-9]+\.[0-9]+)|^{}{SEPARATOR_REGEX}",
                    regex_syntax::escape(env!("CARGO_HOME")),
                )
                .as_ref(),
                "--instr-profile".as_ref(),
                all_profdata.as_os_str(),
                cpp_test_executable.as_os_str(),
                "--object".as_ref(),
                rust_test_executable.as_os_str(),
            ]);

            command
        });

        match self.output_type {
            OutputType::Html => {
                utilities::run_command(add_common_llvm_cov_args(Command::new(llvm_cov).args([
                    "show".as_ref(),
                    "--show-branches".as_ref(),
                    "count".as_ref(),
                    "--show-expansions".as_ref(),
                    "--show-line-counts-or-regions".as_ref(),
                    "--format".as_ref(),
                    "html".as_ref(),
                    "--output-dir".as_ref(),
                    self.output_path.as_os_str(),
                    "--Xdemangler".as_ref(),
                    "rustfilt".as_ref(),
                ])));
            }
            OutputType::Lcov => {
                if let Some(parent_dir) = self.output_path.parent() {
                    fs::create_dir_all(parent_dir).unwrap();
                }

                utilities::run_command_and_redirect_output(
                    add_common_llvm_cov_args(Command::new(llvm_cov).args(["export", "--format", "lcov"])),
                    File::create(self.output_path).unwrap(),
                );
            }
        }
    }
}
