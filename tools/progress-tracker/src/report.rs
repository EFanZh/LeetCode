use crate::html::ElementWriter;
use crate::problems::Problem;
use crate::solutions::{self, Language, Solution};
use git2::Tree;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn make_solution_map(tree: &Tree) -> HashMap<String, Vec<Vec<Solution>>> {
    let mut result = HashMap::<_, Vec<Vec<_>>>::new();

    solutions::list(tree, |solution| {
        match result.entry(solution.problem_id.clone()) {
            Entry::Occupied(entry) => entry.into_mut()[solution.language as usize].push(solution),
            Entry::Vacant(entry) => {
                let mut solutions = Language::list().iter().map(|_| Vec::new()).collect::<Vec<_>>();

                solutions[solution.language as usize].push(solution);

                entry.insert(solutions);
            }
        };
    });

    result
}

fn write_hyper_link<W: Write>(writer: &mut ElementWriter<W>, href: &str, text: &str) {
    writer.add_element_with_attributes("a", [("href", href)]).add_text(text);
}

fn write_problem_link<W: Write>(writer: &mut ElementWriter<W>, problem: &Problem) {
    write_hyper_link(
        writer,
        &format!("https://leetcode.com/problems/{}/", problem.stat.title_slug),
        &problem.stat.title,
    );
}

fn write_solution_link<W: Write>(writer: &mut ElementWriter<W>, solution: &Solution) {
    write_hyper_link(
        writer,
        &format!(
            "https://github.com/EFanZh/LeetCode/blob/master/{}{}",
            solution.solution_root, solution.solution_file
        ),
        &solution.solution_id,
    );
}

fn write_difficulty<W: Write>(writer: &mut ElementWriter<W>, level: u8) {
    for _ in 0..level {
        writer.add_text("★");
    }
}

fn write_html(writer: &mut impl Write, problems: &[Problem], tree: &Tree, progress_chart: &str) {
    const TITLE: &str = "LeetCode Progress Report";

    let solution_map = make_solution_map(tree);

    writeln!(writer, "<!DOCTYPE html>").unwrap();

    // `<html>`.

    let mut html = ElementWriter::with_attributes(writer, "html", [("lang", "en")]);

    // `<head>`.

    let mut head = html.add_element("head");

    head.add_empty_element_with_attributes("meta", [("charset", "utf-8")]);
    head.add_element("title").add_text(TITLE);

    head.add_element("style").add_raw(
        r#"body { font: 14px system-ui, sans-serif; }
h1,h2 { text-align: center; }
figure { display: flex; justify-content: center; }
.detail { border-collapse: collapse; }
.detail>*>tr>* { padding: 0.125em 0.25em; text-align: left; }
.detail>*>tr>*:nth-child(1) { text-align: center; }
.detail>*>tr>*:nth-child(2) { text-align: right; }
.detail>thead { background: white; position: sticky; top: 0; z-index: 1; }
.detail>tbody>tr:nth-child(odd) { background: #eee; }
.detail>tbody ul { margin: 0; padding: 0; list-style-type: none; }
.not-done>td { color: rgba(0, 0, 0, 0.382); }
.not-done>td a:link { color: rgba(0, 0, 255, 0.382); }"#,
    );

    head.close();

    // `<body>`.

    let mut body = html.add_element("body");

    body.add_element("h1").add_text(TITLE);

    write_hyper_link(
        &mut body.add_element_with_attributes("div", [("style", "text-align: center;")]),
        "https://github.com/EFanZh/LeetCode",
        "Source code",
    );

    body.add_element("h2").add_text("Progress Chart");

    body.add_element("figure")
        .add_empty_element_with_attributes("img", [("src", progress_chart), ("alt", "Progress Chart")]);

    body.add_element("h2").add_text("Detail");

    let mut figure = body.add_element("figure");

    // `<table>`.

    let mut table = figure.add_element_with_attributes("table", [("class", "detail")]);

    // `<thead>`.

    {
        let mut table_head = table.add_element("thead");
        let mut thead_tr = table_head.add_element("tr");

        for text in ["Done", "ID", "Title", "Difficulty"] {
            thead_tr.add_element("th").add_text(text);
        }

        for language in Language::list() {
            thead_tr.add_element("th").add_text(&format!("{} Solutions", language));
        }
    }

    // `<tbody>`.

    let mut table_body = table.add_element("tbody");

    for problem in problems {
        if let Some(solution_list) = solution_map.get(&problem.get_id()) {
            let mut tr = table_body.add_element("tr");

            tr.add_element("td").add_text("✔");

            tr.add_element("td")
                .add_text(&problem.stat.frontend_question_id.to_string());

            write_problem_link(&mut tr.add_element("td"), problem);
            write_difficulty(&mut tr.add_element("td"), problem.difficulty.level);

            for solutions in solution_list {
                let mut td = tr.add_element("td");
                let mut ul = td.add_element("ul");

                for solution in solutions {
                    write_solution_link(&mut ul.add_element("li"), solution);
                }
            }
        } else {
            let mut tr = table_body.add_element_with_attributes("tr", [("class", "not-done")]);

            tr.add_element("td");

            tr.add_element("td")
                .add_text(&problem.stat.frontend_question_id.to_string());

            write_problem_link(&mut tr.add_element("td"), problem);
            write_difficulty(&mut tr.add_element("td"), problem.difficulty.level);

            for _ in Language::list() {
                tr.add_element("td");
            }
        }
    }
}

pub fn generate(problems: &[Problem], tree: &Tree, progress_chart: &str, output: &Path) {
    let mut output_file = File::create(output).unwrap();

    write_html(&mut output_file, problems, tree, progress_chart);

    writeln!(output_file).unwrap();
}
