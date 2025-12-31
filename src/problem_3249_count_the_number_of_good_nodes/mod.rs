pub mod dfs;

pub trait Solution {
    fn count_good_nodes(edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]] as &[_], 7),
            (&[[0, 1], [1, 2], [2, 3], [3, 4], [0, 5], [1, 6], [2, 7], [3, 8]], 6),
            (
                &[
                    [0, 1],
                    [1, 2],
                    [1, 3],
                    [1, 4],
                    [0, 5],
                    [5, 6],
                    [6, 7],
                    [7, 8],
                    [0, 9],
                    [9, 10],
                    [9, 12],
                    [10, 11],
                ],
                12,
            ),
            (&[[1, 0], [3, 0], [2, 3]], 3),
        ];

        for (edges, expected) in test_cases {
            assert_eq!(S::count_good_nodes(edges.iter().map(Vec::from).collect()), expected);
        }
    }
}
