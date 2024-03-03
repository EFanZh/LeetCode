pub mod dijkstra;

pub trait Solution {
    fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    5,
                    &[
                        [1, 2, 3],
                        [1, 3, 3],
                        [2, 3, 1],
                        [1, 4, 2],
                        [5, 2, 2],
                        [3, 5, 1],
                        [5, 4, 10],
                    ] as &[_],
                ),
                3,
            ),
            (
                (
                    7,
                    &[
                        [1, 3, 1],
                        [4, 1, 2],
                        [7, 3, 4],
                        [2, 5, 3],
                        [5, 6, 1],
                        [6, 7, 2],
                        [7, 5, 3],
                        [2, 6, 4],
                    ],
                ),
                1,
            ),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(
                S::count_restricted_paths(n, edges.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
