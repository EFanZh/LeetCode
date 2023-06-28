pub mod cumulative_sum;
pub mod sliding_window;

pub trait Solution {
    fn maximum_unique_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[4, 2, 4, 5, 6] as &[_], 17), (&[5, 2, 1, 2, 5, 2, 1, 2, 5], 8)];

        for (nums, expected) in test_cases {
            assert_eq!(S::maximum_unique_subarray(nums.to_vec()), expected);
        }
    }
}
