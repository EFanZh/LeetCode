pub mod iterative;
pub mod partial_iterative;
pub mod recursive;

pub trait Solution {
    fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            &[1, 2, 3] as &[_],
            &[&[] as &[_], &[1], &[1, 2], &[1, 2, 3], &[1, 3], &[2], &[2, 3], &[3]] as &[&[_]],
        )];

        for (nums, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(
                    S::subsets(nums.to_vec())
                        .into_iter()
                        .map(test_utilities::unstable_sorted)
                ),
                expected
            );
        }
    }
}
