pub mod brute_force;

pub trait Solution {
    fn dig_artifacts(n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    2,
                    &[[0, 0, 0, 0], [0, 1, 1, 1]] as &dyn Matrix<_>,
                    &[[0, 0], [0, 1]] as &[_],
                ),
                1,
            ),
            ((2, &[[0, 0, 0, 0], [0, 1, 1, 1]], &[[0, 0], [0, 1], [1, 1]]), 2),
            (
                (
                    6,
                    &[[0, 2, 0, 5], [0, 1, 1, 1], [3, 0, 3, 3], [4, 4, 4, 4], [2, 1, 2, 4]],
                    &[
                        [0, 2],
                        [0, 3],
                        [0, 4],
                        [2, 0],
                        [2, 1],
                        [2, 2],
                        [2, 5],
                        [3, 0],
                        [3, 1],
                        [3, 3],
                        [3, 4],
                        [4, 0],
                        [4, 3],
                        [4, 5],
                        [5, 0],
                        [5, 1],
                        [5, 2],
                        [5, 4],
                        [5, 5],
                    ],
                ),
                0,
            ),
        ];

        for ((n, artifacts, dig), expected) in test_cases {
            assert_eq!(
                S::dig_artifacts(n, artifacts.to_vec(), dig.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
