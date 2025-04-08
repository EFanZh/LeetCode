pub mod binary_heap;

pub trait Solution {
    fn max_score(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 3, 2] as &[_], &[2, 1, 3, 4] as &[_], 3), 12),
            ((&[4, 2, 3, 1, 1], &[7, 5, 10, 9, 6], 1), 30),
        ];

        for ((nums1, nums2, k), expected) in test_cases {
            assert_eq!(S::max_score(nums1.to_vec(), nums2.to_vec(), k), expected);
        }
    }
}
