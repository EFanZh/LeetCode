pub mod iterative;

pub trait Solution {
    fn max_equal_freq(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 2, 1, 1, 5, 3, 3, 5] as &[_], 7),
            (&[1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 5], 13),
            (&[10, 2, 8, 9, 3, 8, 1, 5, 2, 3, 7, 6], 8),
            (&[1, 1], 2),
            (&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9], 10),
            (&[1, 1, 1, 2, 2, 2, 3, 3, 3], 7),
            (&[1], 1),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_equal_freq(nums.to_vec()), expected);
        }
    }
}
