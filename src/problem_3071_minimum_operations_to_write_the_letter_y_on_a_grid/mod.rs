pub mod iterative;

pub trait Solution {
    fn minimum_operations_to_write_y(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 2], [1, 1, 0], [0, 1, 0]] as &dyn Matrix<_>, 3),
            (
                &[
                    [0, 1, 0, 1, 0],
                    [2, 1, 0, 1, 2],
                    [2, 2, 2, 0, 1],
                    [2, 2, 2, 2, 2],
                    [2, 1, 2, 2, 2],
                ],
                12,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::minimum_operations_to_write_y(grid.to_vec()), expected);
        }
    }
}
