pub mod iterative;

pub trait Solution {
    fn rearrange_array(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [&[1, 2, 3, 4, 5] as &[_], &[6, 2, 0, 9, 7]];

        for nums in test_cases {
            let result = S::rearrange_array(nums.to_vec());

            assert_eq!(
                test_utilities::unstable_sorted(result.iter().copied()),
                test_utilities::unstable_sorted(nums.iter().copied()),
            );

            let mut prev_1 = i32::MIN;
            let mut prev_2 = i32::MIN;

            for value in result {
                assert_ne!(prev_1 + value, prev_2 << 1);

                prev_1 = prev_2;
                prev_2 = value;
            }
        }
    }
}
