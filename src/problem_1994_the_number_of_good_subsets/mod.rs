pub mod dynamic_programming;

pub trait Solution {
    fn number_of_good_subsets(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3, 4] as &[_], 6),
            (&[4, 2, 3, 15], 5),
            (&[6, 8, 1, 8, 6, 5, 6, 11, 17], 62),
            (&[9, 7, 20, 20, 2], 3),
            (
                &[
                    5, 10, 1, 26, 24, 21, 24, 23, 11, 12, 27, 4, 17, 16, 2, 6, 1, 1, 6, 8, 13, 30, 24, 20, 2, 19,
                ],
                5368,
            ),
            (
                &[
                    9, 3, 14, 12, 14, 3, 23, 23, 30, 9, 2, 6, 26, 17, 5, 8, 23, 6, 8, 9, 2, 4, 30, 21, 19, 8, 1, 23,
                    22, 26, 17, 20, 5, 15, 18, 20, 22, 2, 15, 8, 21, 9, 20,
                ],
                9958,
            ),
            (&[18, 28, 2, 17, 29, 30, 15, 9, 12], 19),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::number_of_good_subsets(nums.to_vec()), expected);
        }
    }
}
