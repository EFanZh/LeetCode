pub mod backtracking;
pub mod memoized_dynamic_programming;

pub trait Solution {
    fn unique_paths_iii(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]] as &dyn Matrix<_>, 2),
            (&[[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]], 4),
            (&[[0, 1], [2, 0]], 0),
            (&[[0, 0, 0, 0, 0, 0, 2, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 1, 0]], 1),
            (&[[0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, -1, 0, 0, -1, 0, 0, -1, 0]], 0),
            (&[[0, 0, 0, 0, 0], [0, 2, 1, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0]], 8),
            (
                &[
                    [-1, 0, 1, -1, 0],
                    [0, -1, -1, -1, 0],
                    [-1, 0, -1, 0, 2],
                    [-1, 0, 0, -1, -1],
                ],
                0,
            ),
            (
                &[[1, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 0], [0, 0, 0, 0, 2]],
                20,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::unique_paths_iii(grid.to_vec()), expected);
        }
    }
}
