pub mod binary_heap;

pub trait Solution {
    fn minimum_difference(nums: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3, 1, 2] as &[_], -1),
            (&[7, 9, 5, 8, 1, 3], 1),
            (
                &[
                    35, 33, 50, 16, 9, 18, 17, 3, 24, 7, 17, 16, 8, 26, 12, 47, 14, 42, 30, 42, 24, 32, 50, 13, 4, 6,
                    16, 24, 12, 14,
                ],
                -198,
            ),
            (&[1, 2, 3, 9, 8, 7, 4, 5, 6], -18),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::minimum_difference(nums.to_vec()), expected);
        }
    }
}
