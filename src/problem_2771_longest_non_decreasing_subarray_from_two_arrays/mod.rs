pub mod iterative;

pub trait Solution {
    fn max_non_decreasing_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 3, 1] as &[_], &[1, 2, 1] as &[_]), 2),
            ((&[1, 3, 2, 1], &[2, 2, 3, 4]), 4),
            ((&[1, 1], &[2, 2]), 2),
            ((&[4, 2], &[10, 4]), 2),
            ((&[3, 19, 13, 19], &[20, 18, 7, 14]), 2),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::max_non_decreasing_length(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
