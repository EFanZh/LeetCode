pub mod iterative;

pub trait Solution {
    fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[4, 6, 5, 9, 3, 7] as &[_], &[0, 0, 2] as &[_], &[2, 3, 5] as &[_]),
                &[true, false, true] as &[_],
            ),
            (
                (
                    &[-12, -9, -3, -12, -6, 15, 20, -25, -20, -15, -10],
                    &[0, 1, 6, 4, 8, 7],
                    &[4, 4, 9, 7, 9, 10],
                ),
                &[false, true, false, false, true, true],
            ),
            (
                (
                    &[-3, -6, -8, -4, -2, -8, -6, 0, 0, 0, 0],
                    &[5, 4, 3, 2, 4, 7, 6, 1, 7],
                    &[6, 5, 6, 3, 7, 10, 7, 4, 10],
                ),
                &[true, true, true, true, false, true, true, true, true],
            ),
        ];

        for ((nums, l, r), expected) in test_cases {
            assert_eq!(
                S::check_arithmetic_subarrays(nums.to_vec(), l.to_vec(), r.to_vec()),
                expected,
            );
        }
    }
}
