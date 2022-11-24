pub mod dynamic_programming;

pub trait Solution {
    fn count_squares(matrix: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]] as &dyn Matrix<_>, 15),
            (&[[1, 0, 1], [1, 1, 0], [1, 1, 0]], 7),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(S::count_squares(matrix.to_vec()), expected);
        }
    }
}
