pub mod longest_common_subsequence;

pub trait Solution {
    fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 4, 2] as &[_], &[1, 2, 4] as &[_]), 2),
            ((&[2, 5, 1, 2, 5], &[10, 5, 2, 1, 5, 2]), 3),
            ((&[1, 3, 7, 1, 7, 5], &[1, 9, 2, 5, 1]), 2),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::max_uncrossed_lines(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
