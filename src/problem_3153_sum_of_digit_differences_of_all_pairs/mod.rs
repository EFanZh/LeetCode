pub mod iterative;
pub mod iterative_2;

pub trait Solution {
    fn sum_digit_differences(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[13, 23, 12] as &[_], 4), (&[10, 10, 10, 10], 0)];

        for (nums, expected) in test_cases {
            assert_eq!(S::sum_digit_differences(nums.to_vec()), expected);
        }
    }
}
