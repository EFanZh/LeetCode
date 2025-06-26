pub mod boyer_moore_majority_vote;

pub trait Solution {
    fn minimum_index(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 2, 2] as &[_], 2),
            (&[2, 1, 3, 1, 1, 1, 7, 1, 2, 1], 4),
            (&[3, 3, 3, 3, 7, 2, 2], -1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_index(nums.to_vec()), expected);
        }
    }
}
