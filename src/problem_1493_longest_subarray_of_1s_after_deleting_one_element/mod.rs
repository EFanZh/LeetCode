pub mod sliding_window;

pub trait Solution {
    fn longest_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 0, 1] as &[_], 3),
            (&[0, 1, 1, 1, 0, 1, 1, 0, 1], 5),
            (&[1, 1, 1], 2),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::longest_subarray(nums.to_vec()), expected);
        }
    }
}
