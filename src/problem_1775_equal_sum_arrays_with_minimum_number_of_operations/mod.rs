pub mod greedy;

pub trait Solution {
    fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 2, 3, 4, 5, 6] as &[_], &[1, 1, 2, 2, 2, 2] as &[_]), 3),
            ((&[1, 1, 1, 1, 1, 1, 1], &[6]), -1),
            ((&[6, 6], &[1]), 3),
            ((&[5, 6, 4, 3, 1, 2], &[6, 3, 3, 1, 4, 5, 3, 4, 1, 3, 4]), 4),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::min_operations(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
