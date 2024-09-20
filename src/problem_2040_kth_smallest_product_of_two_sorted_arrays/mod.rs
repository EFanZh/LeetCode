pub mod binary_search;

pub trait Solution {
    fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 5] as &[_], &[3, 4] as &[_], 2), 8),
            ((&[-4, -2, 0, 3], &[2, 4], 6), 0),
            ((&[-2, -1, 0, 1, 2], &[-3, -1, 2, 4, 5], 3), -6),
            (
                (
                    &[-10, -9, -8, -5, -3, -2, 1, 2, 4, 8],
                    &[-9, -8, -8, -4, -4, -3, -1, 0, 4],
                    73,
                ),
                32,
            ),
        ];

        for ((nums1, nums2, k), expected) in test_cases {
            assert_eq!(S::kth_smallest_product(nums1.to_vec(), nums2.to_vec(), k), expected);
        }
    }
}
