pub mod dynamic_programming;

pub trait Solution {
    fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]] as &dyn Matrix<_>, 13),
            (&[[7]], 7),
            (
                &[
                    [-73, 61, 43, -48, -36],
                    [3, 30, 27, 57, 10],
                    [96, -76, 84, 59, -15],
                    [5, -49, 76, 31, -7],
                    [97, 91, 61, -46, 67],
                ],
                -192,
            ),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::min_falling_path_sum(grid.to_vec()), expected);
        }
    }
}
