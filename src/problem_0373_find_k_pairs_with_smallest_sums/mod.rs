pub mod bfs;

pub trait Solution {
    fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[1, 7, 11] as &[_], &[2, 4, 6] as &[_], 3),
                &[[1, 2], [1, 4], [1, 6]] as &[_],
            ),
            ((&[1, 1, 2], &[1, 2, 3], 2), &[[1, 1], [1, 1]]),
            ((&[1, 2], &[3], 3), &[[1, 3], [2, 3]]),
            (
                (&[1, 7, 11], &[2, 4, 6], 9),
                &[
                    [1, 2],
                    [1, 4],
                    [1, 6],
                    [7, 2],
                    [7, 4],
                    [7, 6],
                    [11, 2],
                    [11, 4],
                    [11, 6],
                ],
            ),
            (
                (&[2, 7, 11, 19, 23], &[3, 5, 13, 17, 29], 25),
                &[
                    [2, 3],
                    [2, 5],
                    [7, 3],
                    [7, 5],
                    [11, 3],
                    [2, 13],
                    [11, 5],
                    [2, 17],
                    [7, 13],
                    [19, 3],
                    [7, 17],
                    [11, 13],
                    [19, 5],
                    [23, 3],
                    [11, 17],
                    [23, 5],
                    [2, 29],
                    [19, 13],
                    [7, 29],
                    [19, 17],
                    [23, 13],
                    [11, 29],
                    [23, 17],
                    [19, 29],
                    [23, 29],
                ],
            ),
        ];

        for ((nums1, nums2, k), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted_by_key(
                    S::k_smallest_pairs(nums1.to_vec(), nums2.to_vec(), k),
                    |item| (item[0] + item[1], item[0], item[1])
                ),
                expected
            );
        }
    }
}
