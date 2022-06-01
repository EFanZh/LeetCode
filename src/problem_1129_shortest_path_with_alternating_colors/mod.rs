pub mod bfs;

pub trait Solution {
    fn shortest_alternating_paths(n: i32, red_edges: Vec<Vec<i32>>, blue_edges: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    3,
                    &[[0, 1], [1, 2]] as &dyn Matrix<_>,
                    &[] as &[[_; 2]; 0] as &dyn Matrix<_>,
                ),
                &[0, 1, -1] as &[_],
            ),
            ((3, &[[0, 1]], &[[2, 1]]), &[0, 1, -1]),
            ((3, &[[0, 1], [0, 2]], &[[1, 0]]), &[0, 1, 1]),
        ];

        for ((n, red_edges, blue_edges), expected) in test_cases {
            assert_eq!(
                S::shortest_alternating_paths(n, red_edges.to_vec(), blue_edges.to_vec()),
                expected
            );
        }
    }
}
