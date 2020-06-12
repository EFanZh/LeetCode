pub mod iterative;
pub mod recursive;
pub mod recursive_2;

pub trait Solution {
    fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[1, 2, 2] as &[_],
                &[&[] as &[_], &[1], &[1, 2], &[1, 2, 2], &[2], &[2, 2]] as &[&[_]],
            ),
            (
                &[4, 4, 4, 1, 4],
                &[
                    &[],
                    &[1],
                    &[1, 4],
                    &[1, 4, 4],
                    &[1, 4, 4, 4],
                    &[1, 4, 4, 4, 4],
                    &[4],
                    &[4, 4],
                    &[4, 4, 4],
                    &[4, 4, 4, 4],
                ],
            ),
        ];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(
                    S::subsets_with_dup(nums.to_vec())
                        .into_iter()
                        .map(test_utilities::unstable_sorted)
                ),
                expected
            );
        }
    }
}
