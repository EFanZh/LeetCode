pub mod iterative;

pub trait Solution {
    fn max_alternating_sum(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 2, 5, 3] as &[_], 7),
            (&[5, 6, 7, 8], 8),
            (&[6, 2, 1, 2, 4, 5], 10),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_alternating_sum(nums.to_vec()), expected);
        }
    }
}
