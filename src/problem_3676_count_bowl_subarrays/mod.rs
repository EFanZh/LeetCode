pub mod monotonic_stack;

pub trait Solution {
    fn bowl_subarrays(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[2, 5, 3, 1, 4] as &[_], 2),
            (&[5, 1, 2, 3, 4], 3),
            (&[1_000_000_000, 999_999_999, 999_999_998], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::bowl_subarrays(nums.to_vec()), expected);
        }
    }
}
