pub mod sliding_window;

pub trait Solution {
    fn longest_nice_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3, 8, 48, 10] as &[_], 3), (&[3, 1, 5, 11, 13], 1)];

        for (nums, expected) in test_cases {
            assert_eq!(S::longest_nice_subarray(nums.to_vec()), expected);
        }
    }
}
