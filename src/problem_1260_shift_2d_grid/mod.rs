pub mod array_rotation;

pub trait Solution {
    fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]] as &dyn Matrix<_>, 1),
                &[[9, 1, 2], [3, 4, 5], [6, 7, 8]] as &dyn Matrix<_>,
            ),
            (
                (&[[3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10], [12, 0, 21, 13]], 4),
                &[[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]],
            ),
            (
                (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]], 9),
                &[[1, 2, 3], [4, 5, 6], [7, 8, 9]],
            ),
        ];

        for ((grid, k), expected) in test_cases {
            assert_eq!(S::shift_grid(grid.to_vec(), k), expected);
        }
    }
}
