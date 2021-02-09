pub mod greedy;

pub trait Solution {
    fn wiggle_max_length(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 7, 4, 9, 2, 5] as &[_], 6),
            (&[1, 17, 5, 10, 13, 15, 10, 5, 16, 8], 7),
            (&[1, 2, 3, 4, 5, 6, 7, 8, 9], 2),
            (&[], 0),
            (&[84], 1),
            (&[3, 3, 3, 2, 5], 3),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(S::wiggle_max_length(nums.to_vec()), expected);
        }
    }
}
