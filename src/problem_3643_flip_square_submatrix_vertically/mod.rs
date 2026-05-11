pub mod iterative;

pub trait Solution {
    fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]] as &dyn Matrix<_>,
                    1,
                    0,
                    3,
                ),
                &[[1, 2, 3, 4], [13, 14, 15, 8], [9, 10, 11, 12], [5, 6, 7, 16]] as &dyn Matrix<_>,
            ),
            ((&[[3, 4, 2, 3], [2, 3, 4, 2]], 0, 2, 2), &[[3, 4, 4, 2], [2, 3, 2, 3]]),
        ];

        for ((grid, x, y, k), expected) in test_cases {
            assert_eq!(S::reverse_submatrix(grid.to_vec(), x, y, k), expected);
        }
    }
}
