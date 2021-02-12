use git2::{Tree, TreeWalkMode, TreeWalkResult};

pub fn get(tree: &Tree, mut f: impl FnMut(&str, &str)) {
    const ROOT_PREFIX: &str = "src/problem_";
    const ROOT_POSTFIX: &str = "/";
    const SOLUTION_POSTFIX: &str = ".rs";

    tree.walk(TreeWalkMode::PreOrder, |root, entry| {
        let name = entry.name().unwrap();

        if root.starts_with(ROOT_PREFIX)
            && root.ends_with(ROOT_POSTFIX)
            && name.ends_with(SOLUTION_POSTFIX)
            && name != "mod.rs"
        {
            let problem_id = &root[ROOT_PREFIX.len()..root.len() - ROOT_POSTFIX.len()];
            let solution_id = &name[..name.len() - SOLUTION_POSTFIX.len()];

            f(problem_id, solution_id);
        }

        TreeWalkResult::Ok
    })
    .unwrap();
}
