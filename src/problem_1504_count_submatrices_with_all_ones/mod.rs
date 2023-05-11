pub mod monotonic_stack;
pub mod monotonic_stack_2;
pub mod monotonic_stack_3;

pub trait Solution {
    fn num_submat(mat: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 0, 1], [1, 1, 0], [1, 1, 0]] as &dyn Matrix<_>, 13),
            (&[[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]], 24),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::num_submat(mat.to_vec()), expected);
        }
    }
}
