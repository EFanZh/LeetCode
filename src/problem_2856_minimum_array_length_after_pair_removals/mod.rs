pub mod boyer_moore_majority_vote;

pub trait Solution {
    fn min_length_after_removals(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], 0),
            (&[1, 1, 2, 2, 3, 3], 0),
            (&[1_000_000_000, 1_000_000_000], 2),
            (&[2, 3, 4, 4, 4], 1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::min_length_after_removals(nums.to_vec()), expected);
        }
    }
}
