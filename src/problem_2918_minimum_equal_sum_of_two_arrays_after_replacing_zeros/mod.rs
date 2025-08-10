pub mod greedy;

pub trait Solution {
    fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 0, 1, 0] as &[_], &[6, 5, 0] as &[_]), 12),
            ((&[2, 0, 2, 0], &[1, 4]), -1),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::min_sum(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
