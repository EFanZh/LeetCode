pub mod two_pointers;

pub trait Solution {
    fn count_pairs(n: i32, edges: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (4, &[[1, 2], [2, 4], [1, 3], [2, 3], [2, 1]] as &[_], &[2, 3] as &[_]),
                &[6, 5] as &[_],
            ),
            (
                (
                    5,
                    &[[1, 5], [1, 5], [3, 4], [2, 5], [1, 3], [5, 1], [2, 3], [2, 5]],
                    &[1, 2, 3, 4, 5],
                ),
                &[10, 10, 9, 8, 6],
            ),
            (
                (
                    6,
                    &[
                        [5, 2],
                        [3, 5],
                        [4, 5],
                        [1, 5],
                        [1, 4],
                        [3, 5],
                        [2, 6],
                        [6, 4],
                        [5, 6],
                        [4, 6],
                        [6, 2],
                        [2, 6],
                        [5, 4],
                        [6, 1],
                        [6, 1],
                        [2, 5],
                        [1, 3],
                        [1, 3],
                        [4, 5],
                    ],
                    &[6, 9, 2, 1, 2, 3, 6, 6, 6, 5, 9, 7, 0, 4, 5, 9, 1, 18, 8, 9],
                ),
                &[
                    15, 13, 15, 15, 15, 15, 15, 15, 15, 15, 13, 15, 15, 15, 15, 13, 15, 0, 14, 13,
                ],
            ),
        ];

        for ((n, edges, queries), expected) in test_cases {
            assert_eq!(
                S::count_pairs(n, edges.iter().map(Vec::from).collect(), queries.to_vec()),
                expected,
            );
        }
    }
}
