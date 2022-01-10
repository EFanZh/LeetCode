pub mod dijkstra;

pub trait Solution {
    fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 1, 10], [0, 2, 1], [1, 2, 2]] as &dyn Matrix<_>, 6, 3), 13),
            ((&[[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]], 10, 4), 23),
            ((&[[1, 2, 4], [1, 4, 5], [1, 3, 1], [2, 3, 4], [3, 4, 5]], 17, 5), 1),
            ((&[[2, 4, 2], [3, 4, 5], [2, 3, 1], [0, 2, 1], [0, 3, 5]], 14, 5), 18),
        ];

        for ((edges, max_moves, n), expected) in test_cases {
            assert_eq!(S::reachable_nodes(edges.to_vec(), max_moves, n), expected);
        }
    }
}
