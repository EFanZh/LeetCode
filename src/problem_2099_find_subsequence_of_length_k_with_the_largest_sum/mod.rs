pub mod quick_select;

pub trait Solution {
    fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 1, 3, 3] as &[_], 2), 6),
            ((&[-1, -2, 3, 4], 3), 6),
            ((&[3, 4, 3, 3], 2), 7),
        ];

        for ((nums, k), expected) in test_cases {
            let result = S::max_subsequence(nums.to_vec(), k);

            assert!(test_utilities::is_subsequence(&result, nums));
            assert_eq!(result.iter().sum::<i32>(), expected);
        }
    }
}
