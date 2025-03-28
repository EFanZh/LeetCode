#![expect(missing_docs, reason = "unnecessary")]

use git2::Repository;
use http::header;
use problems::Problems;
use reqwest::blocking::Client;
use std::path::Path;
use std::{env, fs};

mod html;
mod problems;
mod progress_chart;
mod report;
mod solutions;

fn get_all_problems() -> Problems {
    Client::new()
        .get("https://leetcode.com/api/problems/algorithms/")
        .header(header::COOKIE, "LEETCODE_SESSION=")
        .send()
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
