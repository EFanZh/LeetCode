pub mod cycle_detection;

pub trait Solution {
    fn is_printable(target_grid: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 1, 1, 1], [1, 2, 2, 1], [1, 2, 2, 1], [1, 1, 1, 1]] as &dyn Matrix<_>,
                true,
            ),
            (&[[1, 1, 1, 1], [1, 1, 3, 3], [1, 1, 3, 4], [5, 5, 1, 4]], true),
            (&[[1, 2, 1], [2, 1, 2], [1, 2, 1]], false),
            (&[[6, 2, 2, 5], [2, 2, 2, 5], [2, 2, 2, 5], [4, 3, 3, 4]], true),
        ];

        for (target_grid, expected) in test_cases {
            assert_eq!(S::is_printable(target_grid.to_vec()), expected);
        }
    }
}
