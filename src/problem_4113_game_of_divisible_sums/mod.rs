pub mod dynamic_programming;

pub trait Solution {
    fn max_difference(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 6, 4] as &[_], 5),
            (&[6, 1, 2, 1, 2], 12),
            (&[3, 3, 4, 5], -3),
            (&[4, 8, 6, 3], 21),
            (&[2, 5, 6, 9, 5], 27),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_difference(nums.to_vec()), expected);
        }
    }
}
