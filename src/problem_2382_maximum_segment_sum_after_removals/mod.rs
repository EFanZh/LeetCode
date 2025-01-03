pub mod reversed_iteration;

pub trait Solution {
    fn maximum_segment_sum(nums: Vec<i32>, remove_queries: Vec<i32>) -> Vec<i64>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 2, 5, 6, 1] as &[_], &[0, 3, 2, 4, 1] as &[_]),
                &[14_i64, 7, 2, 2, 0] as &[_],
            ),
            ((&[3, 2, 11, 1], &[3, 2, 1, 0]), &[16, 5, 3, 0]),
            (
                (
                    &[
                        244, 19, 445, 671, 801, 103, 291, 335, 781, 33, 51, 789, 746, 510, 38, 7, 529, 905,
                    ],
                    &[4, 8, 11, 12, 1, 5, 0, 9, 6, 17, 3, 15, 14, 7, 2, 13, 16, 10],
                ),
                &[
                    5118, 3608, 2735, 1989, 1989, 1989, 1989, 1989, 1989, 1116, 1084, 548, 529, 529, 529, 529, 51, 0,
                ],
            ),
        ];

        for ((nums, remove_queries), expected) in test_cases {
            assert_eq!(S::maximum_segment_sum(nums.to_vec(), remove_queries.to_vec()), expected);
        }
    }
}
