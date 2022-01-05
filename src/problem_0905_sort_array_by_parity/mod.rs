pub mod iterative;

pub trait Solution {
    fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [&[3, 1, 2, 4] as &[_], &[0], &[0, 2, 4], &[1, 0, 3], &[1, 3]];

        for nums in test_cases {
            let result = S::sort_array_by_parity(nums.to_vec());

            assert_eq!(
                test_utilities::unstable_sorted(result.iter().copied()),
                test_utilities::unstable_sorted(nums.iter().copied())
            );

            let split = result
                .iter()
                .position(|&num| num % 2 != 0)
                .unwrap_or_else(|| result.len());

            let (left, right) = result.split_at(split);

            assert!(left.iter().all(|&num| num % 2 == 0));
            assert!(right.iter().all(|&num| num % 2 != 0));
        }
    }
}
