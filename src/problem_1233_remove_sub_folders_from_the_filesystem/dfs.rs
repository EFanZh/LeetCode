pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

enum Folder<'a> {
    Internal(HashMap<&'a str, Self>),
    Leaf(&'a str),
}

impl Solution {
    fn dfs(node: &Folder, result: &mut Vec<String>) {
        match node {
            Folder::Internal(children) => {
                for child in children.values() {
                    Self::dfs(child, result);
                }
            }
            &Folder::Leaf(path) => result.push(path.to_string()),
        }
    }

    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut root = Folder::Internal(HashMap::new());

        'outer: for path in folder.iter().map(String::as_str) {
            let mut node = &mut root;

            for name in path.split('/').skip(1) {
                match node {
                    Folder::Internal(children) => {
                        node = children.entry(name).or_insert_with(|| Folder::Internal(HashMap::new()));
                    }
                    Folder::Leaf(_) => continue 'outer,
                }
            }

            *node = Folder::Leaf(path);
        }

        let mut result = Vec::new();

        Self::dfs(&root, &mut result);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        Self::remove_subfolders(folder)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
