pub mod hash_map;

pub trait Solution {
    fn max_balanced_subarray(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, 3, 2, 0] as &[_], 4),
            (&[3, 2, 8, 5, 4, 14, 9, 15], 8),
            (&[0], 0),
            (&[3, 4, 2, 1, 3, 1], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_balanced_subarray(nums.to_vec()), expected);
        }
    }
}
