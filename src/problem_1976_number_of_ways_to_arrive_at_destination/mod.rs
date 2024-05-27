pub mod dijkstra;
pub mod dijkstra_2;

pub trait Solution {
    fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    7,
                    &[
                        [0, 6, 7],
                        [0, 1, 2],
                        [1, 2, 3],
                        [1, 3, 3],
                        [6, 3, 3],
                        [3, 5, 1],
                        [6, 5, 1],
                        [2, 5, 1],
                        [0, 4, 5],
                        [4, 6, 2],
                    ] as &[_],
                ),
                4,
            ),
            ((2, &[[1, 0, 10]]), 1),
        ];

        for ((n, roads), expected) in test_cases {
            assert_eq!(S::count_paths(n, roads.iter().map(Vec::from).collect()), expected);
        }
    }
}
