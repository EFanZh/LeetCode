pub mod interning;

pub trait Solution {
    fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&["a"] as &[_], &["c"], &["d"], &["a", "b"], &["c", "b"], &["d", "a"]] as &[&[_]],
                &[&["d"] as &[_], &["d", "a"]] as &[&[_]],
            ),
            (
                &[
                    &["a"],
                    &["c"],
                    &["a", "b"],
                    &["c", "b"],
                    &["a", "b", "x"],
                    &["a", "b", "x", "y"],
                    &["w"],
                    &["w", "y"],
                ],
                &[&["a"], &["a", "b"], &["c"], &["c", "b"]],
            ),
            (
                &[&["a", "b"], &["c", "d"], &["c"], &["a"]],
                &[&["a"], &["a", "b"], &["c"], &["c", "d"]],
            ),
        ];

        for (paths, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::delete_duplicate_folder(
                    paths
                        .iter()
                        .map(|account| account.iter().copied().map(str::to_string).collect())
                        .collect()
                )),
                expected,
            );
        }
    }
}
