pub mod iterative;

pub trait Solution {
    fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[0, 1, 1], [1, 0, 1], [0, 0, 1]] as &dyn Matrix<_>,
                &[[0, 0, 4], [0, 0, 4], [-2, -2, 2]] as &dyn Matrix<_>,
            ),
            (&[[1, 1, 1], [1, 1, 1]], &[[5, 5, 5], [5, 5, 5]]),
        ];

        for (grid, expected) in test_cases {
            assert_eq!(S::ones_minus_zeros(grid.to_vec()), expected);
        }
    }
}
