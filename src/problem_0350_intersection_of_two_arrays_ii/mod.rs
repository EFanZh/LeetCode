pub mod single_map;

pub trait Solution {
    fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 2, 1] as &[_], &[2, 2] as &[_]), &[2, 2] as &[_]),
            ((&[4, 9, 5], &[9, 4, 9, 8, 4]), &[4, 9]),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::intersect(nums1.to_vec(), nums2.to_vec())),
                expected
            );
        }
    }
}
