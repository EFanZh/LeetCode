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
    clippy::print_stderr,
    clippy::print_stdout,
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
    // clippy::wildcard_dependencies,
)]
#![allow(clippy::non_ascii_literal)]

use git2::Repository;
use problems::Problems;
use reqwest::blocking;
use std::path::Path;
use std::{env, fs};

mod html;
mod problems;
mod progress_chart;
mod report;
mod solutions;

fn get_all_problems() -> Problems {
    blocking::get("https://leetcode.com/api/problems/algorithms/")
        .unwrap()
        .json()
        .unwrap()
}

fn generate_report(repository: &Path, target: &Path) {
    let mut problems = get_all_problems();
    let repository = Repository::open(repository).unwrap();

    problems.retain_free();
    problems.problems.sort_by_key(|p| p.stat.frontend_question_id);

    fs::create_dir_all(target).unwrap();

    // Generate progress chart.

    progress_chart::draw(&repository, &problems, &target.join("progress.svg"));

    // Generate report.

    let tree = repository.head().unwrap().peel_to_tree().unwrap();

    report::generate(&problems.problems, &tree, "progress.svg", &target.join("index.html"));
}

fn main() {
    let mut args = env::args();

    args.next().unwrap();

    let repository = args.next().unwrap();
    let target = args.next().unwrap();

    generate_report(repository.as_ref(), target.as_ref());
}
