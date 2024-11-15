pub mod prefix_sums;

pub trait Solution {
    fn maximums_spliced_array(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[60, 60, 60] as &[_], &[10, 90, 10] as &[_]), 210),
            ((&[20, 40, 20, 70, 30], &[50, 20, 50, 40, 20]), 220),
            ((&[7, 11, 13], &[1, 1, 1]), 31),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::maximums_spliced_array(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
