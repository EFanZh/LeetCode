pub mod iterative;

pub trait Solution {
    fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[4, 6, 7, 7] as &[_],
                &[
                    &[4, 6] as &[_],
                    &[4, 6, 7],
                    &[4, 6, 7, 7],
                    &[4, 7],
                    &[4, 7, 7],
                    &[6, 7],
                    &[6, 7, 7],
                    &[7, 7],
                ] as &[&[_]],
            ),
            (
                &[1, 2, 3, 4, 1, 1, 1],
                &[
                    &[1, 1],
                    &[1, 1, 1],
                    &[1, 1, 1, 1],
                    &[1, 2],
                    &[1, 2, 3],
                    &[1, 2, 3, 4],
                    &[1, 2, 4],
                    &[1, 3],
                    &[1, 3, 4],
                    &[1, 4],
                    &[2, 3],
                    &[2, 3, 4],
                    &[2, 4],
                    &[3, 4],
                ],
            ),
            (&[4, 3, 2, 1], &[]),
            (&[3, 4, 3, 4], &[&[3, 3], &[3, 3, 4], &[3, 4], &[3, 4, 4], &[4, 4]]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(
                    S::find_subsequences(nums.to_vec())
                        .into_iter()
                        .map(test_utilities::unstable_sorted)
                ),
                expected
            );
        }
    }
}
