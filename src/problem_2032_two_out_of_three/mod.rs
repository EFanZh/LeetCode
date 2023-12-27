pub mod iterative;

pub trait Solution {
    fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 1, 3, 2] as &[_], &[2, 3] as &[_], &[3] as &[_]), &[2, 3] as &[_]),
            ((&[3, 1], &[2, 3], &[1, 2]), &[1, 2, 3]),
            ((&[1, 2, 2], &[4, 3, 3], &[5]), &[]),
        ];

        for ((nums1, nums2, nums3), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::two_out_of_three(nums1.to_vec(), nums2.to_vec(), nums3.to_vec())),
                expected,
            );
        }
    }
}
