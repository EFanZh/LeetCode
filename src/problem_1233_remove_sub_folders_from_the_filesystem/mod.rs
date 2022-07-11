pub mod dfs;

pub trait Solution {
    fn remove_subfolders(folder: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"] as &[_],
                &["/a", "/c/d", "/c/f"] as &[_],
            ),
            (&["/a", "/a/b/c", "/a/b/d"], &["/a"]),
            (&["/a/b/c", "/a/b/ca", "/a/b/d"], &["/a/b/c", "/a/b/ca", "/a/b/d"]),
        ];

        for (folder, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::remove_subfolders(
                    folder.iter().copied().map(str::to_string).collect()
                )),
                expected
            );
        }
    }
}
