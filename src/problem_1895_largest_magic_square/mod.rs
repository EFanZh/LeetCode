pub mod prefix_sum;

pub trait Solution {
    fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[7, 1, 4, 5, 6], [2, 5, 1, 6, 4], [1, 5, 4, 3, 2], [1, 2, 7, 3, 4]] as &dyn Matrix<_>,
                3,
            ),
            (&[[5, 1, 3, 1], [9, 3, 3, 1], [1, 3, 3, 8]], 2),
            (
                &[
                    [1, 4, 6, 3, 4, 18, 14, 20],
                    [2, 8, 2, 19, 12, 17, 16, 17],
                    [11, 3, 5, 7, 8, 17, 10, 14],
                    [13, 9, 12, 21, 28, 5, 12, 19],
                    [2, 12, 7, 27, 9, 11, 18, 20],
                    [5, 14, 18, 8, 10, 17, 24, 26],
                    [15, 17, 2, 14, 16, 23, 25, 7],
                    [5, 3, 10, 15, 22, 29, 6, 13],
                ],
                5,
            ),
            (&[[8, 1, 6], [3, 5, 7], [4, 9, 2], [7, 10, 9]], 3),
            (&[[1]], 1),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::largest_magic_square(grid.to_vec()), expected);
        }
    }
}
