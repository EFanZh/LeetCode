pub mod cheating;
pub mod single_set;
pub mod two_sets;

pub trait Solution {
    fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 2, 1] as &[_], &[2, 2] as &[_]), &[2] as &[_]),
            ((&[4, 9, 5], &[9, 4, 9, 8, 4]), &[4, 9]),
        ];

        for ((nums1, nums2), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::intersection(nums1.to_vec(), nums2.to_vec())),
                expected
            );
        }
    }
}
