pub mod bfs;
pub mod dynamic_programming;

pub trait Solution {
    fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[0, 0, 0], [0, 1, 0], [0, 0, 0]] as &dyn Matrix<_>,
                &[[0, 0, 0], [0, 1, 0], [0, 0, 0]] as &dyn Matrix<_>,
            ),
            (&[[0, 0, 0], [0, 1, 0], [1, 1, 1]], &[[0, 0, 0], [0, 1, 0], [1, 2, 1]]),
            (
                &[
                    [0, 1, 0, 1, 1],
                    [1, 1, 0, 0, 1],
                    [0, 0, 0, 1, 0],
                    [1, 0, 1, 1, 1],
                    [1, 0, 0, 0, 1],
                ],
                &[
                    [0, 1, 0, 1, 2],
                    [1, 1, 0, 0, 1],
                    [0, 0, 0, 1, 0],
                    [1, 0, 1, 1, 1],
                    [1, 0, 0, 0, 1],
                ],
            ),
            (
                &[
                    [1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
                    [0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
                    [0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
                    [1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
                    [0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
                    [0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
                    [1, 0, 0, 0, 1, 1, 1, 1, 0, 1],
                    [1, 1, 1, 1, 1, 1, 1, 0, 1, 0],
                    [1, 1, 1, 1, 0, 1, 0, 0, 1, 1],
                ],
                &[
                    [1, 0, 1, 1, 0, 0, 1, 0, 0, 1],
                    [0, 1, 1, 0, 1, 0, 1, 0, 1, 1],
                    [0, 0, 1, 0, 1, 0, 0, 1, 0, 0],
                    [1, 0, 1, 0, 1, 1, 1, 1, 1, 1],
                    [0, 1, 0, 1, 1, 0, 0, 0, 0, 1],
                    [0, 0, 1, 0, 1, 1, 1, 0, 1, 0],
                    [0, 1, 0, 1, 0, 1, 0, 0, 1, 1],
                    [1, 0, 0, 0, 1, 2, 1, 1, 0, 1],
                    [2, 1, 1, 1, 1, 2, 1, 0, 1, 0],
                    [3, 2, 2, 1, 0, 1, 0, 0, 1, 1],
                ],
            ),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::update_matrix(mat.to_vec()), expected);
        }
    }
}
