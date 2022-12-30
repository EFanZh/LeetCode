pub mod iterative;

pub trait Solution {
    fn has_valid_path(grid: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2, 4, 3], [6, 5, 2]] as &dyn Matrix<_>, true),
            (&[[1, 2, 1], [1, 2, 1]], false),
            (&[[1, 1, 2]], false),
            (&[[1]], true),
            (&[[4, 3, 3], [6, 5, 2]], false),
            (
                &[
                    [3, 4, 6, 1, 3, 1, 5, 5, 1, 4, 4, 2, 5],
                    [5, 5, 1, 4, 5, 3, 2, 6, 2, 2, 6, 3, 3],
                    [2, 2, 1, 4, 6, 2, 4, 4, 1, 4, 6, 3, 1],
                    [5, 3, 4, 5, 1, 5, 6, 1, 6, 6, 5, 1, 2],
                    [4, 4, 1, 2, 2, 2, 6, 2, 3, 6, 6, 5, 1],
                    [2, 2, 1, 5, 2, 1, 5, 6, 2, 5, 6, 5, 6],
                    [1, 2, 4, 3, 4, 1, 3, 4, 3, 1, 4, 3, 1],
                    [1, 1, 5, 2, 5, 5, 4, 1, 1, 5, 1, 5, 1],
                    [4, 6, 6, 2, 6, 4, 6, 2, 3, 3, 2, 5, 4],
                    [5, 4, 1, 6, 2, 6, 5, 3, 4, 4, 6, 6, 2],
                    [6, 6, 6, 3, 1, 4, 3, 3, 4, 2, 2, 5, 4],
                    [4, 6, 6, 4, 6, 3, 1, 3, 3, 1, 1, 5, 1],
                    [5, 3, 5, 3, 1, 4, 6, 3, 5, 5, 2, 5, 5],
                ],
                false,
            ),
            (
                &[
                    [1, 3, 2, 3, 6, 3, 2, 1, 6, 2, 4, 5, 5, 2, 3, 2, 2, 4],
                    [5, 5, 5, 1, 4, 4, 6, 3, 6, 1, 2, 6, 2, 2, 6, 6, 2, 1],
                    [6, 6, 3, 2, 5, 1, 1, 1, 2, 3, 3, 5, 5, 6, 6, 2, 3, 3],
                    [3, 3, 5, 6, 3, 6, 2, 3, 4, 3, 2, 2, 1, 1, 2, 5, 1, 6],
                    [1, 1, 6, 3, 6, 4, 3, 4, 1, 5, 3, 2, 4, 4, 2, 1, 5, 6],
                    [3, 3, 3, 1, 6, 4, 1, 6, 3, 2, 1, 1, 6, 2, 6, 3, 4, 2],
                    [5, 2, 4, 2, 4, 4, 5, 3, 2, 2, 5, 6, 3, 1, 1, 2, 2, 6],
                    [5, 6, 6, 4, 2, 4, 2, 1, 2, 5, 3, 1, 2, 1, 2, 6, 4, 2],
                    [4, 5, 6, 2, 6, 1, 2, 4, 5, 3, 1, 6, 3, 1, 6, 5, 4, 4],
                    [5, 6, 2, 1, 2, 6, 6, 5, 5, 3, 5, 4, 2, 1, 3, 3, 2, 1],
                    [3, 5, 1, 4, 2, 3, 6, 1, 6, 2, 4, 1, 5, 2, 5, 5, 4, 6],
                    [3, 4, 3, 3, 6, 3, 5, 4, 1, 4, 6, 5, 3, 5, 4, 6, 4, 5],
                    [6, 4, 2, 3, 1, 6, 6, 3, 5, 1, 4, 3, 4, 4, 1, 1, 6, 1],
                    [6, 3, 4, 1, 5, 3, 6, 3, 1, 1, 2, 5, 1, 3, 2, 2, 1, 2],
                    [2, 6, 5, 4, 4, 3, 2, 6, 2, 1, 2, 1, 2, 3, 3, 1, 5, 4],
                    [6, 4, 5, 1, 2, 6, 1, 3, 1, 6, 5, 3, 2, 3, 6, 6, 2, 5],
                    [6, 5, 5, 3, 1, 3, 5, 6, 3, 2, 5, 1, 2, 3, 2, 5, 5, 1],
                    [6, 6, 5, 2, 4, 4, 6, 5, 1, 6, 2, 4, 4, 6, 2, 5, 4, 6],
                    [5, 6, 2, 2, 6, 1, 2, 6, 4, 6, 5, 5, 2, 2, 2, 5, 6, 1],
                ],
                false,
            ),
            (
                &[
                    [1, 1, 1, 1, 6],
                    [1, 1, 1, 1, 2],
                    [1, 1, 1, 1, 2],
                    [1, 1, 1, 1, 2],
                    [1, 1, 1, 1, 2],
                ],
                false,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::has_valid_path(grid.to_vec()), expected);
        }
    }
}
