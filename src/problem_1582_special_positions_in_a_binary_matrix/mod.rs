pub mod iterative;

pub trait Solution {
    fn num_special(mat: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0, 0], [0, 0, 1], [1, 0, 0]] as &dyn Matrix<_>, 1),
            (&[[1, 0, 0], [0, 1, 0], [0, 0, 1]], 3),
            (&[[0, 0, 0, 1], [1, 0, 0, 0], [0, 1, 1, 0], [0, 0, 0, 0]], 2),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::num_special(mat.to_vec()), expected);
        }
    }
}
