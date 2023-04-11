pub mod iterative;

pub trait Solution {
    fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2, 3], [4, 5, 6], [7, 8, 9]] as &dyn Matrix<_>, 25),
            (&[[1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1], [1, 1, 1, 1]], 8),
            (&[[5]], 5),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::diagonal_sum(mat.to_vec()), expected);
        }
    }
}
