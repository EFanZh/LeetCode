pub mod bfs;

pub trait Solution {
    fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 7, 11] as &[_], &[2, 4, 6] as &[_], 3),
                &[[1, 2], [1, 4], [1, 6]] as &[_],
            ),
            ((&[1, 1, 2], &[1, 2, 3], 2), &[[1, 1], [1, 1]]),
            ((&[1, 2], &[3], 3), &[[1, 3], [2, 3]]),
        ];

        for ((nums1, nums2, k), expected) in test_cases.iter().copied() {
            assert_eq!(S::k_smallest_pairs(nums1.to_vec(), nums2.to_vec(), k), expected);
        }
    }
}
