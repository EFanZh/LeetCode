pub mod iterative;

pub trait Solution {
    fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[0, 1], [1, 0], [4, 0], [0, 4], [3, 3], [2, 4]] as &[_], [0, 0]),
                &[[0, 1], [1, 0], [3, 3]] as &[_],
            ),
            (
                (&[[0, 0], [1, 1], [2, 2], [3, 4], [3, 5], [4, 4], [4, 5]], [3, 3]),
                &[[2, 2], [3, 4], [4, 4]],
            ),
            (
                (
                    &[
                        [5, 6],
                        [7, 7],
                        [2, 1],
                        [0, 7],
                        [1, 6],
                        [5, 1],
                        [3, 7],
                        [0, 3],
                        [4, 0],
                        [1, 2],
                        [6, 3],
                        [5, 0],
                        [0, 4],
                        [2, 2],
                        [1, 1],
                        [6, 4],
                        [5, 4],
                        [0, 0],
                        [2, 6],
                        [4, 5],
                        [5, 2],
                        [1, 4],
                        [7, 5],
                        [2, 3],
                        [0, 5],
                        [4, 2],
                        [1, 0],
                        [2, 7],
                        [0, 1],
                        [4, 6],
                        [6, 1],
                        [0, 6],
                        [4, 3],
                        [1, 7],
                    ],
                    [3, 4],
                ),
                &[[2, 3], [1, 4], [1, 6], [3, 7], [4, 3], [5, 4], [4, 5]],
            ),
        ];

        for ((queens, king), expected) in test_cases {
            assert_eq!(
                S::queens_attackthe_king(queens.iter().map(Vec::from).collect(), king.to_vec()),
                expected,
            );
        }
    }
}
