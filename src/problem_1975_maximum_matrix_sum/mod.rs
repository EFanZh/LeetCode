pub mod greedy;

pub trait Solution {
    fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, -1], [-1, 1]] as &dyn Matrix<_>, 4),
            (&[[1, 2, 3], [-1, -2, -3], [1, 2, 3]], 16),
            (&[[-1, 0, -1], [-2, 1, 3], [3, 2, 2]], 15),
            (&[[2, 9, 3], [5, 4, -4], [1, 7, 1]], 34),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(S::max_matrix_sum(matrix.to_vec()), expected);
        }
    }
}
