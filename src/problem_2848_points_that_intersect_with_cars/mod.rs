pub mod sweep_line;

pub trait Solution {
    fn number_of_points(nums: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[3, 6], [1, 5], [4, 7]] as &dyn Matrix<_>, 7), (&[[1, 3], [5, 8]], 7)];

        for (nums, expected) in test_cases {
            assert_eq!(S::number_of_points(nums.to_vec()), expected);
        }
    }
}
