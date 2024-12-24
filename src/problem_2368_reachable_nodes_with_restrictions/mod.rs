pub mod bfs;

pub trait Solution {
    fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    7,
                    &[[0, 1], [1, 2], [3, 1], [4, 0], [0, 5], [5, 6]] as &[_],
                    &[4, 5] as &[_],
                ),
                4,
            ),
            ((7, &[[0, 1], [0, 2], [0, 5], [0, 4], [3, 2], [6, 5]], &[4, 2, 1]), 3),
        ];

        for ((n, edges, restricted), expected) in test_cases {
            assert_eq!(
                S::reachable_nodes(n, edges.iter().map(Vec::from).collect(), restricted.to_vec()),
                expected,
            );
        }
    }
}
