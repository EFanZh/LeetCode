pub mod partition_by_bit;

pub trait Solution {
    fn single_number(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 1, 3, 2, 5] as &[_], [3, 5]),
            (&[-1, 0], [-1, 0]),
            (&[0, 1], [0, 1]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::single_number(nums.to_vec())),
                expected
            );
        }
    }
}
