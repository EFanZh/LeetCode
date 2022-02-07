pub mod iterative;

pub trait Solution {
    fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [&[4, 2, 5, 7] as &[_], &[2, 3], &[3, 1, 4, 2]];

        for nums in test_cases {
            let result = S::sort_array_by_parity_ii(nums.to_vec());

            assert_eq!(
                test_utilities::unstable_sorted(result.iter().copied()),
                test_utilities::unstable_sorted(nums.iter().copied())
            );

            for (i, &num) in result.iter().enumerate() {
                assert_eq!(num % 2 == 0, i % 2 == 0);
            }
        }
    }
}
