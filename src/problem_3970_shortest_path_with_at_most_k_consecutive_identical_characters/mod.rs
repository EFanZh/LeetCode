pub mod dijkstra;

pub trait Solution {
    fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[0, 1, 1], [1, 2, 1], [0, 2, 3]] as &[_], "aab", 1), 3),
            ((3, &[[0, 1, 1], [1, 2, 1], [0, 2, 3]], "aab", 2), 2),
            ((3, &[[0, 1, 1], [1, 2, 1]], "aaa", 2), -1),
        ];

        for ((n, edges, labels, k), expected) in test_cases {
            assert_eq!(
                S::shortest_path(n, edges.iter().map(Vec::from).collect(), labels.to_string(), k),
                expected,
            );
        }
    }
}
