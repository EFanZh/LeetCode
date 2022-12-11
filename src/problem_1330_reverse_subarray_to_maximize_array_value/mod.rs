pub mod mathematical;

pub trait Solution {
    fn max_value_after_reverse(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 3, 1, 5, 4] as &[_], 10),
            (&[2, 4, 9, 24, 2, 1, 10], 68),
            (&[2, 5, 1, 3, 4], 11),
            (&[4, 3, 1, 5, 2], 11),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_value_after_reverse(nums.to_vec()), expected);
        }
    }
}
