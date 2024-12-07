pub mod greedy;

pub trait Solution {
    fn maximum_score(scores: Vec<i32>, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[5, 2, 9, 8, 4] as &[_],
                    &[[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]] as &[_],
                ),
                24,
            ),
            ((&[9, 20, 6, 4, 11, 12], &[[0, 3], [5, 3], [2, 4], [1, 3]]), -1),
            (
                (
                    &[18, 6, 4, 9, 8, 2],
                    &[
                        [0, 1],
                        [0, 2],
                        [0, 3],
                        [0, 4],
                        [0, 5],
                        [1, 2],
                        [1, 3],
                        [1, 4],
                        [1, 5],
                        [2, 3],
                        [2, 4],
                        [2, 5],
                        [3, 4],
                        [3, 5],
                        [4, 5],
                    ],
                ),
                41,
            ),
        ];

        for ((scores, edges), expected) in test_cases {
            assert_eq!(
                S::maximum_score(scores.to_vec(), edges.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
