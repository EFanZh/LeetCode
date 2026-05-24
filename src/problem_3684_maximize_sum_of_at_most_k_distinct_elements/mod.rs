pub mod hash_set;

pub trait Solution {
    fn max_k_distinct(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[84, 93, 100, 77, 90] as &[_], 3), &[90, 93, 100] as &[_]),
            ((&[84, 93, 100, 77, 93], 3), &[84, 93, 100]),
            ((&[1, 1, 1, 2, 2, 2], 6), &[1, 2]),
        ];

        for ((nums, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::max_k_distinct(nums.to_vec(), k)),
                expected
            );
        }
    }
}
