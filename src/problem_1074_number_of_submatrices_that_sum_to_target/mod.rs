pub mod iterative;

pub trait Solution {
    fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 1, 0], [1, 1, 1], [0, 1, 0]] as &dyn Matrix<_>, 0), 4),
            ((&[[1, -1], [-1, 1]], 0), 5),
            ((&[[904]], 0), 0),
        ];

        for ((matrix, target), expected) in test_cases {
            assert_eq!(S::num_submatrix_sum_target(matrix.to_vec(), target), expected);
        }
    }
}
