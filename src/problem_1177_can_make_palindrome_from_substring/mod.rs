pub mod iterative;

pub trait Solution {
    fn can_make_pali_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    "abcda",
                    &[[3, 3, 0], [1, 2, 0], [0, 3, 1], [0, 3, 2], [0, 4, 1]] as &[_],
                ),
                &[true, false, false, true, true] as &[_],
            ),
            (("lyb", &[[0, 1, 0], [2, 2, 1]]), &[false, true]),
            (
                (
                    "hunu",
                    &[
                        [1, 1, 1],
                        [2, 3, 0],
                        [3, 3, 1],
                        [0, 3, 2],
                        [1, 3, 3],
                        [2, 3, 1],
                        [3, 3, 1],
                        [0, 3, 0],
                        [1, 1, 1],
                        [2, 3, 0],
                        [3, 3, 1],
                        [0, 3, 1],
                        [1, 1, 1],
                    ],
                ),
                &[
                    true, false, true, true, true, true, true, false, true, false, true, true, true,
                ],
            ),
        ];

        for ((s, queries), expected) in test_cases {
            assert_eq!(
                S::can_make_pali_queries(s.to_string(), queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
