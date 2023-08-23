pub mod mathematical;

pub trait Solution {
    fn find_center(edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1, 2], [2, 3], [4, 2]] as &[_], 2),
            (&[[1, 2], [5, 1], [1, 3], [1, 4]], 1),
            (&[[1, 5], [2, 5], [3, 5], [4, 5], [5, 5]], 5),
            (
                &[
                    [10, 1],
                    [10, 2],
                    [3, 10],
                    [10, 4],
                    [5, 10],
                    [10, 6],
                    [10, 7],
                    [8, 10],
                    [10, 9],
                    [10, 11],
                    [12, 10],
                    [10, 13],
                    [14, 10],
                ],
                10,
            ),
        ];

        for (edges, expected) in test_cases {
            assert_eq!(S::find_center(edges.iter().copied().map(Vec::from).collect()), expected);
        }
    }
}
