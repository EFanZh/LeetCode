pub mod iterative;

pub trait Solution {
    fn matrix_sum(nums: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[7, 2, 1], [6, 4, 2], [6, 5, 3], [3, 2, 1]] as &dyn Matrix<_>, 15),
            (&[[1]], 1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::matrix_sum(nums.to_vec()), expected);
        }
    }
}
