pub mod bfs;

pub trait Solution {
    fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    &[1, 2, 2, 3, 5] as &[_],
                    &[3, 2, 3, 4, 4],
                    &[2, 4, 5, 3, 1],
                    &[6, 7, 1, 4, 5],
                    &[5, 1, 1, 2, 4],
                ] as &[&[_]],
                &[[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]] as &[_],
            ),
            (&[], &[]),
        ];

        for (graph, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::pacific_atlantic(graph.iter().map(|row| row.to_vec()).collect())),
                expected
            );
        }
    }
}
