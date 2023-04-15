pub mod hash_map;

pub trait Solution {
    fn num_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[7, 4] as &[_], &[5, 2, 8, 9] as &[_]), 1),
            ((&[1, 1], &[1, 1, 1]), 9),
            ((&[7, 7, 8, 3], &[1, 2, 9, 7]), 2),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::num_triplets(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
