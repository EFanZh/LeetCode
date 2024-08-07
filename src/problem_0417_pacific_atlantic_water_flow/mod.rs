pub mod bfs;

pub trait Solution {
    fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::{self, Matrix};

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    [1, 2, 2, 3, 5],
                    [3, 2, 3, 4, 4],
                    [2, 4, 5, 3, 1],
                    [6, 7, 1, 4, 5],
                    [5, 1, 1, 2, 4],
                ] as &dyn Matrix<_>,
                &[[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]] as &[_],
            ),
            (&[] as &[[_; 0]; 0], &[]),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::pacific_atlantic(graph.to_vec())),
                expected,
            );
        }
    }
}
