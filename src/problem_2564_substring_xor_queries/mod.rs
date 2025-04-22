pub mod sliding_window;

pub trait Solution {
    fn substring_xor_queries(s: String, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("101101", &[[0, 5], [1, 2]] as &[_]), &[[0, 2], [2, 3]] as &[_]),
            (("0101", &[[12, 8]]), &[[-1, -1]]),
            (("1", &[[4, 5]]), &[[0, 0]]),
            (
                (
                    "111010110",
                    &[
                        [4, 2],
                        [3, 3],
                        [6, 4],
                        [9, 9],
                        [10, 28],
                        [0, 470],
                        [5, 83],
                        [10, 28],
                        [8, 15],
                        [6, 464],
                        [0, 3],
                        [5, 8],
                        [7, 7],
                        [8, 8],
                        [6, 208],
                        [9, 15],
                        [2, 2],
                        [9, 95],
                    ],
                ),
                &[
                    [1, 3],
                    [3, 3],
                    [2, 3],
                    [3, 3],
                    [4, 8],
                    [0, 8],
                    [2, 8],
                    [4, 8],
                    [0, 2],
                    [0, 8],
                    [0, 1],
                    [1, 4],
                    [3, 3],
                    [3, 3],
                    [1, 8],
                    [1, 3],
                    [3, 3],
                    [2, 8],
                ],
            ),
        ];

        for ((s, queries), expected) in test_cases {
            assert_eq!(
                S::substring_xor_queries(s.to_string(), queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
