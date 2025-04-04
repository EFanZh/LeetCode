#![expect(missing_docs, reason = "unnecessary")]

use regex::Regex;
use std::path::PathBuf;
use std::{env, fs};

fn to_all_caps_case(id: &str) -> String {
    let mut result = String::with_capacity(id.len());

    for c in id.chars() {
        if c == '-' {
            result.push('_');
        } else {
            result.extend(c.to_uppercase());
        }
    }

    result
}

fn to_title_case(id: &str) -> String {
    let mut result = String::new();

    for component in id.split('-') {
        let mut chars = component.chars();

        result.extend(chars.next().unwrap().to_uppercase());
        result.extend(chars);
    }

    result
}

fn check_regex(re: &str, text: &str) {
    assert!(
        Regex::new(re).unwrap().is_match(text),
        "regex = {re:?}, text = {text:?}",
    );
}

#[expect(clippy::print_stdout, reason = "by design")]
fn main() {
    let project_path = PathBuf::from(env::args_os().nth(1).unwrap());
    let leet_code_include_path = project_path.join("include").join("leet-code");
    let leet_code_tests_path = project_path.join("tests").join("leet-code");

    for problem_id in fs::read_dir(&leet_code_include_path).unwrap().filter_map(|entry| {
        entry
            .unwrap()
            .path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_prefix("problem-")
            .map(str::to_string)
    }) {
        println!("Checking problem “{problem_id}” ...");

        let problem_include_guard = to_all_caps_case(&problem_id);
        let problem_namespace = problem_id.replace('-', "_");

        println!("    Checking tests ...");

        let solution_tests_header_regex = format!(
            "(?s)^\
                #ifndef LEET_CODE_PROBLEM_{problem_include_guard}_TESTS_H\r?\n\
                #define LEET_CODE_PROBLEM_{problem_include_guard}_TESTS_H\r?\n\r?\n\
                ((#include (<[^>]+>|\"[^\"]+\")\r?\n)+\r?\n)?\
                namespace leet_code::problem_{problem_namespace}::tests \\{{\r?\n\
                template <class .>\r?\n\
                void run\\(\\) \\{{\
                \r?\n.*\
                \\}}\r?\n\
                \\}} // namespace leet_code::problem_{problem_namespace}::tests\r?\n\r?\n\
                #endif // LEET_CODE_PROBLEM_{problem_include_guard}_TESTS_H\r?\n\
            $",
            problem_include_guard = problem_include_guard.as_str(),
            problem_namespace = problem_namespace.as_str(),
        );

        let problem_test_path = leet_code_tests_path.join(format!("problem-{problem_id}"));
        let problem_tests_source = fs::read_to_string(problem_test_path.join("tests.h")).unwrap();

        check_regex(&solution_tests_header_regex, &problem_tests_source);

        println!("    Checking solutions ...");

        let problem_path = leet_code_include_path.join(format!("problem-{problem_id}"));
        let problem_name = to_title_case(&problem_id);

        for solution_id in fs::read_dir(&problem_path)
            .unwrap()
            .map(|entry| entry.unwrap().path().file_stem().unwrap().to_str().unwrap().to_string())
        {
            println!("        Checking solution “{solution_id}” ...");
            println!("            Checking solution ...");

            let solution_namespace = solution_id.replace('-', "_");

            let solution_regex = format!(
                "(?s)^\
                    #ifndef LEET_CODE_PROBLEM_{problem_include_guard}_{solution_include_guard}_H\r?\n\
                    #define LEET_CODE_PROBLEM_{problem_include_guard}_{solution_include_guard}_H\r?\n\r?\n\
                    ((#include <[^>]+>\r?\n)+\r?\n)?\
                    namespace leet_code::problem_{problem_namespace}::{solution_namespace} \\{{\r?\n\
                    .*\
                    \\}} // namespace leet_code::problem_{problem_namespace}::{solution_namespace}\r?\n\r?\n\
                    #endif // LEET_CODE_PROBLEM_{problem_include_guard}_{solution_include_guard}_H\r?\n\
                $",
                problem_include_guard = problem_include_guard.as_str(),
                solution_include_guard = to_all_caps_case(&solution_id),
                problem_namespace = problem_namespace.as_str(),
                solution_namespace = solution_namespace.as_str(),
            );

            let solution_source = fs::read_to_string(problem_path.join(format!("{solution_id}.h"))).unwrap();

            check_regex(&solution_regex, &solution_source);

            println!("            Checking tests ...");

            let solution_tests_regex = format!(
                "(?s)^\
                    #include \"tests.h\"\r?\n\
                    #include <leet-code/problem-{problem_id}/{solution_id}.h>\r?\n\r?\n\
                    namespace leet_code::problem_{problem_namespace}::tests \\{{\r?\n\
                    TEST\\(Problem{problem_name}, {solution_name}\\) \\{{\r?\n\
                    [ ]   tests::run<{solution_namespace}::[^>]+>\\(\\);\r?\n\
                    \\}}\r?\n\
                    \\}} // namespace leet_code::problem_{problem_namespace}::tests\r?\n\
                $",
                problem_id = problem_id,
                solution_id = solution_id,
                problem_namespace = problem_namespace,
                problem_name = problem_name,
                solution_name = to_title_case(&solution_id),
                solution_namespace = solution_namespace,
            );

            let solution_tests_source =
                fs::read_to_string(problem_test_path.join(format!("{solution_id}.cpp"))).unwrap();

            check_regex(&solution_tests_regex, &solution_tests_source);
        }
    }
}
