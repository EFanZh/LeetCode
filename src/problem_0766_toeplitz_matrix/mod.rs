pub mod iterative;

pub trait Solution {
    fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 3, 4], [5, 1, 2, 3], [9, 5, 1, 2]] as &dyn Matrix<_>, true),
            (&[[1, 2], [2, 2]], false),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(S::is_toeplitz_matrix(matrix.to_vec()), expected);
        }
    }
}
