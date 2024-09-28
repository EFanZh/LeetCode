pub mod dynamic_programming;
pub mod prefix_sums;

pub trait Solution {
    fn max_trailing_zeros(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    [23, 17, 15, 3, 20],
                    [8, 1, 20, 27, 11],
                    [9, 4, 6, 2, 21],
                    [40, 9, 1, 10, 6],
                    [22, 7, 4, 5, 3],
                ] as &dyn Matrix<_>,
                3,
            ),
            (&[[4, 3, 2], [7, 6, 1], [8, 8, 8]], 0),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::max_trailing_zeros(grid.to_vec()), expected);
        }
    }
}
