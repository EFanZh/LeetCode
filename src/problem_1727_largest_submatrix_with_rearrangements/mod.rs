pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 0, 1], [1, 1, 1], [1, 0, 1]] as &dyn Matrix<_>, 4),
            (&[[1, 0, 1, 0, 1]], 3),
            (&[[1, 1, 0], [1, 0, 1]], 2),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(S::largest_submatrix(matrix.to_vec()), expected);
        }
    }
}
