pub mod sliding_window;
pub mod sliding_window_2;

pub trait Solution {
    fn min_operations(nums: Vec<i32>, x: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 4, 2, 3] as &[_], 5), 2),
            ((&[5, 6, 7, 8, 9], 4), -1),
            ((&[3, 2, 20, 1, 1, 3], 10), 5),
            ((&[1, 1], 3), -1),
            ((&[5, 2, 3, 1, 1], 5), 1),
            (
                (
                    &[
                        8828, 9581, 49, 9818, 9974, 9869, 9991, 10000, 10000, 10000, 9999, 9993, 9904, 8819, 1231, 6309,
                    ],
                    134_365,
                ),
                16,
            ),
        ];

        for ((nums, x), expected) in test_cases {
            assert_eq!(S::min_operations(nums.to_vec(), x), expected);
        }
    }
}
