pub mod fenwick_tree;

pub trait Solution {
    fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[2, 0, 1, 3] as &[_], &[0, 1, 2, 3] as &[_]), 1),
            ((&[4, 0, 1, 3, 2], &[4, 1, 0, 2, 3]), 4),
        ];

        for ((nums1, nums2), expected) in test_cases {
            assert_eq!(S::good_triplets(nums1.to_vec(), nums2.to_vec()), expected);
        }
    }
}
