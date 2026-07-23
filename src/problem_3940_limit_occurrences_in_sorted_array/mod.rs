pub mod iterative;

pub trait Solution {
    fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 1, 2, 2, 3] as &[_], 2), &[1, 1, 2, 2, 3] as &[_]),
            ((&[1, 2, 3], 1), &[1, 2, 3]),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::limit_occurrences(nums.to_vec(), k)),
                expected
            );
        }
    }
}
