pub mod iterative;

pub trait Solution {
    fn max_score_indices(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 0, 1, 0] as &[_], &[2, 4] as &[_]),
            (&[0, 0, 0], &[3]),
            (&[1, 1], &[0]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::max_score_indices(nums.to_vec())),
                expected,
            );
        }
    }
}
