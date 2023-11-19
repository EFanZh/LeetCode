pub mod kruskal;

pub trait Solution {
    fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    #[allow(clippy::too_many_lines)] // Expected.
    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    5,
                    &[
                        [0, 1, 1],
                        [1, 2, 1],
                        [2, 3, 2],
                        [0, 3, 2],
                        [0, 4, 3],
                        [3, 4, 3],
                        [1, 4, 6],
                    ] as &[_],
                ),
                (&[0, 1] as &[_], &[2, 3, 4, 5] as &[_]),
            ),
            ((4, &[[0, 1, 1], [1, 2, 1], [2, 3, 1], [0, 3, 1]]), (&[], &[0, 1, 2, 3])),
            (
                (
                    14,
                    &[
                        [0, 1, 13],
                        [0, 2, 6],
                        [2, 3, 13],
                        [3, 4, 4],
                        [0, 5, 11],
                        [4, 6, 14],
                        [4, 7, 8],
                        [2, 8, 6],
                        [4, 9, 6],
                        [7, 10, 4],
                        [5, 11, 3],
                        [6, 12, 7],
                        [12, 13, 9],
                        [7, 13, 2],
                        [5, 13, 10],
                        [0, 6, 4],
                        [2, 7, 3],
                        [0, 7, 8],
                        [1, 12, 9],
                        [10, 12, 11],
                        [1, 2, 7],
                        [1, 3, 10],
                        [3, 10, 6],
                        [6, 10, 4],
                        [4, 8, 5],
                        [1, 13, 4],
                        [11, 13, 8],
                        [2, 12, 10],
                        [5, 8, 1],
                        [3, 7, 6],
                        [7, 12, 12],
                        [1, 7, 9],
                        [5, 9, 1],
                        [2, 13, 10],
                        [10, 11, 4],
                        [3, 5, 10],
                        [6, 11, 14],
                        [5, 12, 3],
                        [0, 8, 13],
                        [8, 9, 1],
                        [3, 6, 8],
                        [0, 3, 4],
                        [2, 9, 6],
                        [0, 11, 4],
                        [2, 5, 14],
                        [4, 11, 2],
                        [7, 11, 11],
                        [1, 11, 6],
                        [2, 10, 12],
                        [0, 13, 4],
                        [3, 9, 9],
                        [4, 12, 3],
                        [6, 7, 10],
                        [6, 8, 13],
                        [9, 11, 3],
                        [1, 6, 2],
                        [2, 4, 12],
                        [0, 10, 3],
                        [3, 12, 1],
                        [3, 8, 12],
                        [1, 8, 6],
                        [8, 13, 2],
                        [10, 13, 12],
                        [9, 13, 11],
                        [2, 11, 14],
                        [5, 10, 9],
                        [5, 6, 10],
                        [2, 6, 9],
                        [4, 10, 7],
                        [3, 13, 10],
                        [4, 13, 3],
                        [3, 11, 9],
                        [7, 9, 14],
                        [6, 9, 5],
                        [1, 5, 12],
                        [4, 5, 3],
                        [11, 12, 3],
                        [0, 4, 8],
                        [5, 7, 8],
                        [9, 12, 13],
                        [8, 12, 12],
                        [1, 10, 6],
                        [1, 9, 9],
                        [7, 8, 9],
                        [9, 10, 13],
                        [8, 11, 3],
                        [6, 13, 7],
                        [0, 12, 10],
                        [1, 4, 8],
                        [8, 10, 2],
                    ],
                ),
                (
                    &[13, 16, 45, 55, 57, 58, 61, 89],
                    &[10, 15, 23, 25, 28, 32, 37, 39, 51, 54, 70, 75, 76, 85],
                ),
            ),
        ];

        for ((n, edges), (expected_critical_edges, expected_pseudo_critical_edges)) in test_cases {
            let result = S::find_critical_and_pseudo_critical_edges(n, edges.iter().map(Vec::from).collect());
            let [actual_critical_edges, actual_pseudo_critical_edges]: [_; 2] = result.try_into().ok().unwrap();

            assert_eq!(
                test_utilities::unstable_sorted(actual_critical_edges),
                expected_critical_edges,
            );

            assert_eq!(
                test_utilities::unstable_sorted(actual_pseudo_critical_edges),
                expected_pseudo_critical_edges,
            );
        }
    }
}
