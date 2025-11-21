pub mod hash_map;

pub trait Solution {
    fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[1, 3, 4] as &[_], &[1, 3, 4] as &[_], 1), 5),
            ((&[1, 2, 4, 12], &[2, 4], 3), 2),
            ((&[1], &[8], 1), 0),
        ];

        for ((nums1, nums2, k), expected) in test_cases {
            assert_eq!(S::number_of_pairs(nums1.to_vec(), nums2.to_vec(), k), expected);
        }
    }
}
