pub mod iterative;

pub trait Solution {
    fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[1, 2, -1], [4, -1, 6], [7, 8, 9]] as &dyn Matrix<_>,
                &[[1, 2, 9], [4, 8, 6], [7, 8, 9]] as &dyn Matrix<_>,
            ),
            (&[[3, -1], [5, 2]], &[[3, 2], [5, 2]]),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(S::modified_matrix(matrix.to_vec()), expected);
        }
    }
}
