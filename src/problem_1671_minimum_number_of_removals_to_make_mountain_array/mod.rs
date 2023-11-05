pub mod longest_increasing_subsequence;

pub trait Solution {
    fn minimum_mountain_removals(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 3, 1] as &[_], 0),
            (&[2, 1, 1, 5, 6, 2, 3, 1], 3),
            (&[9, 8, 1, 7, 6, 5, 4, 3, 2, 1], 2),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_mountain_removals(nums.to_vec()), expected);
        }
    }
}
