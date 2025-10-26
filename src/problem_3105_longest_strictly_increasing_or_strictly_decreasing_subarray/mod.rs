pub mod iterative;

pub trait Solution {
    fn longest_monotonic_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 4, 3, 3, 2] as &[_], 2), (&[3, 3, 3, 3], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::longest_monotonic_subarray(nums.to_vec()), expected);
        }
    }
}
