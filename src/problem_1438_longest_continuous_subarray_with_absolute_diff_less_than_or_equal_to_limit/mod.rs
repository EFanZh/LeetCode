pub mod monotonic_stacks;

pub trait Solution {
    fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[8, 2, 4, 7] as &[_], 4), 2),
            ((&[10, 1, 2, 4, 7, 2], 5), 4),
            ((&[4, 2, 2, 2, 4, 4, 2, 2], 0), 3),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(S::longest_subarray(nums.to_vec(), k), expected);
        }
    }
}
