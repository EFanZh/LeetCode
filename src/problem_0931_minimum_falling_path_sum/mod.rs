pub mod dynamic_programming;

pub trait Solution {
    fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2, 1, 3], [6, 5, 4], [7, 8, 9]] as &dyn Matrix<_>, 13),
            (&[[-19, 57], [-40, -5]], -59),
        ];

        for (matrix, expected) in test_cases {
            assert_eq!(S::min_falling_path_sum(matrix.to_vec()), expected);
        }
    }
}
