pub mod hash_map;

pub trait Solution {
    fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    "root/a 1.txt(abcd) 2.txt(efgh)",
                    "root/c 3.txt(abcd)",
                    "root/c/d 4.txt(efgh)",
                    "root 4.txt(efgh)",
                ] as &[_],
                &[
                    &["root/4.txt", "root/a/2.txt", "root/c/d/4.txt"] as &[_],
                    &["root/a/1.txt", "root/c/3.txt"],
                ] as &[&[_]],
            ),
            (
                &[
                    "root/a 1.txt(abcd) 2.txt(efgh)",
                    "root/c 3.txt(abcd)",
                    "root/c/d 4.txt(efgh)",
                ],
                &[&["root/a/1.txt", "root/c/3.txt"], &["root/a/2.txt", "root/c/d/4.txt"]],
            ),
            (
                &[
                    "root/a 1.txt(abcd) 2.txt(efsfgh)",
                    "root/c 3.txt(abdfcd)",
                    "root/c/d 4.txt(efggdfh)",
                ],
                &[],
            ),
        ];

        for (paths, expected) in test_cases {
            let paths = paths.iter().copied().map(str::to_string).collect();

            assert_eq!(
                test_utilities::unstable_sorted(
                    S::find_duplicate(paths)
                        .into_iter()
                        .map(test_utilities::unstable_sorted)
                ),
                expected
            );
        }
    }
}
