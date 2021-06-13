use crate::html::Writer;
use crate::problems::Problem;
use crate::solutions::{self, Language, Solution};
use git2::Tree;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
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

fn write_hyper_link(writer: &mut Writer, href: &str, text: &str) {
    writer.element("a", &[("href", href)], |w| w.text(text));
}

fn write_problem_link(writer: &mut Writer, problem: &Problem) {
    write_hyper_link(
        writer,
        &format!("https://leetcode.com/problems/{}/", problem.stat.title_slug),
        &problem.stat.title,
    );
}

fn write_solution_link(writer: &mut Writer, solution: &Solution) {
    write_hyper_link(
        writer,
        &format!(
            "https://github.com/EFanZh/LeetCode/blob/master/{}{}",
            solution.solution_root, solution.solution_file
        ),
        &solution.solution_id,
    );
}

fn write_difficulty(writer: &mut Writer, level: u8) {
    for _ in 0..level {
        writer.text("★");
    }
}

pub fn generate(problems: &[Problem], tree: &Tree, progress_chart: &str, output: &Path) {
    const TITLE: &str = "LeetCode Progress Report";

    let solution_map = make_solution_map(tree);
    let mut result = String::from("<!DOCTYPE html>\n");
    let mut html_writer = Writer::on(&mut result);

    html_writer.element("html", &[("lang", "en")], |w| {
        w.element("head", &[], |w| {
            w.empty_element("meta", &[("charset", "utf-8")]);
            w.element("title", &[], |w| w.text(TITLE));
            w.element("style", &[], |w| {
                w.raw(
                    r#"body { font: 14px system-ui, sans-serif; }
h1,h2 { text-align: center; }
figure { display: flex; justify-content: center; }
.detail { border-collapse: collapse; }
.detail>*>tr>* { padding: 0.125em 0.25em; text-align: left; }
.detail>*>tr>*:nth-child(1) { text-align: center; }
.detail>*>tr>*:nth-child(2) { text-align: right; }
.detail>thead>tr>th { background: white; position: sticky; top: 0; z-index: 1; }
.detail>tbody>tr:nth-child(odd) { background: #eee; }
.detail>tbody>tr>td>ul { margin: 0; padding: 0; list-style-type: none; }
.not-done>td { opacity: 0.382; }"#,
                );
            });
        });
        w.element("body", &[], |w| {
            w.element("h1", &[], |w| w.text(TITLE));
            w.element("div", &[("style", "text-align: center;")], |w| {
                write_hyper_link(w, "https://github.com/EFanZh/LeetCode", "Source code");
            });
            w.element("h2", &[], |w| w.text("Progress Chart"));
            w.element("figure", &[], |w| {
                w.empty_element("img", &[("src", progress_chart), ("alt", "Progress Chart")]);
            });
            w.element("h2", &[], |w| w.text("Detail"));
            w.element("figure", &[], |w| {
                w.element("table", &[("class", "detail")], |w| {
                    w.element("thead", &[], |w| {
                        w.element("tr", &[], |w| {
                            w.element("th", &[], |w| w.text("Done"));
                            w.element("th", &[], |w| w.text("ID"));
                            w.element("th", &[], |w| w.text("Title"));
                            w.element("th", &[], |w| w.text("Difficulty"));

                            for language in &Language::list() {
                                w.element("th", &[], |w| w.text(&format!("{} Solutions", language)));
                            }
                        });
                    });
                    w.element("tbody", &[], |w| {
                        for problem in problems {
                            if let Some(solution_list) = solution_map.get(&problem.get_id()) {
                                w.element("tr", &[], |w| {
                                    w.element("td", &[], |w| w.text("✔"));
                                    w.element("td", &[], |w| {
                                        w.text(&problem.stat.frontend_question_id.to_string());
                                    });
                                    w.element("td", &[], |w| write_problem_link(w, problem));
                                    w.element("td", &[], |w| write_difficulty(w, problem.difficulty.level));

                                    for solutions in solution_list {
                                        w.element("td", &[], |w| {
                                            w.element("ul", &[], |w| {
                                                for solution in solutions {
                                                    w.element("li", &[], |w| {
                                                        write_solution_link(w, solution);
                                                    });
                                                }
                                            });
                                        });
                                    }
                                });
                            } else {
                                w.element("tr", &[("class", "not-done")], |w| {
                                    w.element("td", &[], |_| {});
                                    w.element("td", &[], |w| w.text(&problem.stat.frontend_question_id.to_string()));
                                    w.element("td", &[], |w| write_problem_link(w, problem));
                                    w.element("td", &[], |w| write_difficulty(w, problem.difficulty.level));

                                    for _ in &Language::list() {
                                        w.element("td", &[], |_| {});
                                    }
                                });
                            }
                        }
                    });
                });
            });
        });
    });

    result.push('\n');

    fs::write(output, result).unwrap();
}
