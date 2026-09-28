pub mod iterative;

pub trait Solution {
    fn cyclic_shift(n: i32, grid: Vec<Vec<i32>>, row_shift: Vec<i32>, col_shift: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (2, &[[1, 2], [3, 4]] as &dyn Matrix<_>, &[1, 0] as &[_], &[0, 1] as &[_]),
                &[[2, 4], [3, 1]] as &dyn Matrix<_>,
            ),
            (
                (3, &[[1, 2, 3], [4, 5, 6], [7, 8, 9]], &[1, 2, 0], &[2, 2, 1]),
                &[[7, 8, 5], [2, 3, 9], [6, 4, 1]],
            ),
        ];

        for ((n, grid, row_shift, col_shift), expected) in test_cases {
            assert_eq!(
                S::cyclic_shift(n, grid.to_vec(), row_shift.to_vec(), col_shift.to_vec()),
                expected,
            );
        }
    }
}
