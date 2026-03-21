pub mod iterative;

pub trait Solution {
    fn sort_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 7, 3], [9, 8, 2], [4, 5, 6]] as &dyn Matrix<_>,
                &[[8, 2, 3], [9, 6, 7], [4, 5, 1]] as &dyn Matrix<_>,
            ),
            (&[[0, 1], [1, 2]], &[[2, 1], [1, 0]]),
            (&[[1]], &[[1]]),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::sort_matrix(grid.to_vec()), expected);
        }
    }
}
