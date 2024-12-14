use crate::data_structures::ListNode;

pub mod iterative;

pub trait Solution {
    fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::{self, Matrix};

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (3, 5, &[3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0] as &[_]),
                &[[3, 0, 2, 6, 8], [5, 0, -1, -1, 1], [5, 2, 4, 9, 7]] as &dyn Matrix<_>,
            ),
            ((1, 4, &[0, 1, 2]), &[[0, 1, 2, -1]]),
            ((1, 8, &[2, 11, 13, 21, 21, 1, 8, 12]), &[[2, 11, 13, 21, 21, 1, 8, 12]]),
            (
                (10, 8, &[483, 100, 904, 632, 267, 352, 386, 887, 753]),
                &[
                    [483, 100, 904, 632, 267, 352, 386, 887],
                    [-1, -1, -1, -1, -1, -1, -1, 753],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                    [-1, -1, -1, -1, -1, -1, -1, -1],
                ],
            ),
            (
                (10, 1, &[8, 24, 5, 21, 10, 11, 11, 12, 6, 17]),
                &[[8], [24], [5], [21], [10], [11], [11], [12], [6], [17]],
            ),
            (
                (
                    10,
                    5,
                    &[47, 708, 1, 918, 227, 334, 115, 863, 636, 769, 117, 557, 940, 54, 83],
                ),
                &[
                    [47, 708, 1, 918, 227],
                    [-1, -1, -1, -1, 334],
                    [-1, -1, -1, -1, 115],
                    [-1, -1, -1, -1, 863],
                    [-1, -1, -1, -1, 636],
                    [-1, -1, -1, -1, 769],
                    [-1, -1, -1, -1, 117],
                    [-1, -1, -1, -1, 557],
                    [-1, -1, -1, -1, 940],
                    [-1, -1, -1, 83, 54],
                ],
            ),
            (
                (
                    9,
                    6,
                    &[
                        995, 348, 36, 516, 333, 627, 248, 422, 13, 225, 764, 311, 405, 695, 698, 83, 145, 783, 478,
                    ],
                ),
                &[
                    [995, 348, 36, 516, 333, 627],
                    [-1, -1, -1, -1, -1, 248],
                    [-1, -1, -1, -1, -1, 422],
                    [-1, -1, -1, -1, -1, 13],
                    [-1, -1, -1, -1, -1, 225],
                    [-1, -1, -1, -1, -1, 764],
                    [-1, -1, -1, -1, -1, 311],
                    [-1, -1, -1, -1, -1, 405],
                    [478, 783, 145, 83, 698, 695],
                ],
            ),
            ((2, 3, &[1, 2, 3, 4, 5, 6]), &[[1, 2, 3], [6, 5, 4]]),
            ((3, 2, &[1, 2, 3, 4, 5, 6]), &[[1, 2], [6, 3], [5, 4]]),
        ];

        for ((m, n, head), expected) in test_cases {
            assert_eq!(
                S::spiral_matrix(m, n, test_utilities::make_list(head.iter().copied())),
                expected,
            );
        }
    }
}