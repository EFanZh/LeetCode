#![warn(
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    // missing_docs,
    noop_method_call,
    pointer_structural_match,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences,
    // clippy::cargo_common_metadata,
    clippy::clone_on_ref_ptr,
    clippy::cognitive_complexity,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::empty_line_after_outer_attr,
    clippy::fallible_impl_from,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::imprecise_flops,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::multiple_inherent_impl,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::panic_in_result_fn,
    clippy::path_buf_push_overwrite,
    clippy::pedantic,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::todo,
    clippy::trivial_regex,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::use_debug,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    // clippy::wildcard_dependencies
)]
#![allow(clippy::non_ascii_literal)]

use regex::Regex;
use std::env;
use std::fs;
use std::path::PathBuf;

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
        "regex = {:?}, text = {:?}",
        re,
        text
    );
}

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
        println!("Checking problem “{}” ...", problem_id);

        let problem_include_guard = to_all_caps_case(&problem_id);
        let problem_namespace = problem_id.replace('-', "_");

        println!("    Checking tests ...");

        let solution_tests_regex = format!(
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

        let problem_test_path = leet_code_tests_path.join(format!("problem-{}", problem_id));
        let problem_tests_source = fs::read_to_string(problem_test_path.join("tests.h")).unwrap();

        check_regex(&solution_tests_regex, &problem_tests_source);

        println!("    Checking solutions ...");

        let problem_path = leet_code_include_path.join(format!("problem-{}", problem_id));
        let problem_name = to_title_case(&problem_id);

        for solution_id in fs::read_dir(&problem_path)
            .unwrap()
            .map(|entry| entry.unwrap().path().file_stem().unwrap().to_str().unwrap().to_string())
        {
            println!("        Checking solution “{}” ...", solution_id);
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

            let solution_source = fs::read_to_string(problem_path.join(format!("{}.h", solution_id))).unwrap();

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
                solution_namespace = solution_namespace
            );

            let solution_tests_source =
                fs::read_to_string(problem_test_path.join(format!("{}.cpp", solution_id))).unwrap();

            check_regex(&solution_tests_regex, &solution_tests_source);
        }
    }
}
