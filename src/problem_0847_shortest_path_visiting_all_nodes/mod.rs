pub mod bfs;

pub trait Solution {
    fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[1, 2, 3] as &[_], &[0], &[0], &[0]] as &[&[_]], 4),
            (&[&[1], &[0, 2, 4], &[1, 3, 4], &[2], &[1, 2]], 4),
            (&[&[1], &[0, 2, 4], &[1, 3], &[2], &[1, 5], &[4]], 6),
            (
                &[
                    &[6, 9],
                    &[6, 8],
                    &[6, 7],
                    &[6, 10],
                    &[8],
                    &[10],
                    &[0, 1, 2, 3, 8],
                    &[2, 8, 9],
                    &[1, 4, 6, 7],
                    &[0, 7, 10],
                    &[3, 5, 9],
                ],
                11,
            ),
            (&[&[]], 0),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(
                S::shortest_path_length(graph.iter().copied().map(<[_]>::to_vec).collect()),
                expected
            );
        }
    }
}
