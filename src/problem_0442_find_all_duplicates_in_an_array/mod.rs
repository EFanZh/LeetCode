pub mod highest_bit_as_marker;
pub mod pigeonhole;

pub trait Solution {
    fn find_duplicates(nums: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[4, 3, 2, 7, 8, 2, 3, 1] as &[_], &[2, 3] as &[_]),
            (&[1, 1], &[1]),
            (&[3, 4, 2, 4], &[4]),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_duplicates(nums.to_vec())),
                expected
            );
        }
    }
}
