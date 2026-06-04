pub mod iterative;

pub trait Solution {
    fn longest_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 1, 1, 1, 2, 3, 5, 1] as &[_], 5),
            (&[5, 2, 7, 9, 16], 5),
            (&[1_000_000_000, 1_000_000_000, 1_000_000_000], 2),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::longest_subarray(nums.to_vec()), expected);
        }
    }
}
