pub mod fenwick_tree;
pub mod fenwick_tree_2;
pub mod merge_sort;

pub trait Solution {
    fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, diff: i32) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[3, 2, 5] as &[_], &[2, 2, 1] as &[_], 1), 3),
            ((&[3, -1], &[-2, 2], -1), 0),
            ((&[-4, -4, 4, -1, -2, 5], &[-2, 2, -1, 4, 4, 3], 1), 9),
        ];

        for ((nums1, nums2, diff), expected) in test_cases {
            assert_eq!(S::number_of_pairs(nums1.to_vec(), nums2.to_vec(), diff), expected);
        }
    }
}
