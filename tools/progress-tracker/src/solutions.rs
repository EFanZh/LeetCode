use git2::{Tree, TreeWalkMode, TreeWalkResult};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Language {
    Rust = 0,
    Cpp = 1,
}

impl Language {
    pub fn list() -> [Self; 2] {
        [Self::Rust, Self::Cpp]
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Language::Rust => "Rust",
            Language::Cpp => "C++",
        })
    }
}

pub struct Solution {
    pub language: Language,
    pub problem_id: String,
    pub solution_id: String,
    pub solution_root: String,
    pub solution_file: String,
}

pub fn list(tree: &Tree, mut f: impl FnMut(Solution)) {
    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        let name = entry.name().unwrap();

        if let Some(problem_id) = root
            .strip_prefix("c++/include/leet-code/problem-")
            .and_then(|s| s.strip_suffix('/'))
        {
            if let Some(solution_id) = name.strip_suffix(".h") {
                f(Solution {
                    language: Language::Cpp,
                    problem_id: problem_id.to_string(),
                    solution_id: solution_id.to_string(),
                    solution_root: root.to_string(),
                    solution_file: name.to_string(),
                });
            }
        } else if let Some(problem_id) = root.strip_prefix("src/problem_").and_then(|s| s.strip_suffix('/')) {
            if let Some(solution_id) = name.strip_suffix(".rs") {
                if name != "mod.rs" {
                    f(Solution {
                        language: Language::Rust,
                        problem_id: problem_id.replace('_', "-"),
                        solution_id: solution_id.to_string(),
                        solution_root: root.to_string(),
                        solution_file: name.to_string(),
                    });
                }
            }
        }

        TreeWalkResult::Ok
    })
    .unwrap();
}
