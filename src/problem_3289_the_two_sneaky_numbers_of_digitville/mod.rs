pub mod pigeonhole_principle;

pub trait Solution {
    fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[0, 1, 1, 0] as &[_], &[0, 1] as &[_]),
            (&[0, 3, 2, 1, 3, 2], &[2, 3]),
            (&[7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2], &[4, 5]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::get_sneaky_numbers(nums.to_vec())),
                expected,
            );
        }
    }
}
