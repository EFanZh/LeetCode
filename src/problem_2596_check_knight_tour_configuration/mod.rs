pub mod iterative;

pub trait Solution {
    fn check_valid_grid(grid: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    [0, 11, 16, 5, 20],
                    [17, 4, 19, 10, 15],
                    [12, 1, 8, 21, 6],
                    [3, 18, 23, 14, 9],
                    [24, 13, 2, 7, 22],
                ] as &dyn Matrix<_>,
                true,
            ),
            (&[[0, 3, 6], [5, 8, 1], [2, 7, 4]], false),
            (
                &[
                    [24, 11, 22, 17, 4],
                    [21, 16, 5, 12, 9],
                    [6, 23, 10, 3, 18],
                    [15, 20, 1, 8, 13],
                    [0, 7, 14, 19, 2],
                ],
                false,
            ),
            (
                &[
                    [20, 15, 10, 3, 22],
                    [9, 4, 21, 16, 11],
                    [14, 19, 6, 23, 2],
                    [5, 8, 1, 12, 17],
                    [0, 13, 18, 7, 24],
                ],
                false,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::check_valid_grid(grid.to_vec()), expected);
        }
    }
}
