use git2::Repository;
use problems::Problems;
use reqwest::blocking;
use std::env;
use std::fs;
use std::path::Path;

mod html;
mod problems;
mod progress_chart;
mod report;
mod solutions;

fn get_all_problems() -> Problems {
    blocking::get("https://leetcode.com/api/problems/algorithms/")
        .unwrap()
        .json::<Problems>()
        .unwrap()
}

fn generate_report<P: AsRef<Path>>(repository: P, target: P) {
    let mut problems = get_all_problems();
    let repository = Repository::open(repository).unwrap();

    problems.retain_free();
    problems.problems.sort_by_key(|p| p.stat.frontend_question_id);

    fs::create_dir_all(target.as_ref()).unwrap();

    // Generate progress chart.

    progress_chart::draw(&repository, &problems, target.as_ref().join("progress.svg"));

    // Generate report.

    let tree = repository.find_reference("HEAD").unwrap().peel_to_tree().unwrap();

    report::generate(
        &problems.problems,
        &tree,
        "progress.svg",
        target.as_ref().join("index.html"),
    );
}

fn main() {
    let mut args = env::args();
    let _ = args.next().unwrap();
    let repository = args.next().unwrap();
    let target = args.next().unwrap();

    generate_report(repository, target);
}
