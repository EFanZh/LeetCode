pub mod dynamic_programming;

pub trait Solution {
    fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1, 2, 3] as &[_], &[&[1, 2] as &[_], &[1, 3]] as &[&[_]]),
            (&[1, 2, 4, 8], &[&[1, 2, 4, 8]]),
            (&[1], &[&[1]]),
        ];

        for (nums, expected) in test_cases {
            let result = test_utilities::unstable_sorted(S::largest_divisible_subset(nums.to_vec()));

            assert!(expected.contains(&result.as_slice()));
        }
    }
}
