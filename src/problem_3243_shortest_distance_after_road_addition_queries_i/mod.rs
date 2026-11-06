pub mod bfs;

pub trait Solution {
    fn shortest_distance_after_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((5, &[[2, 4], [0, 2], [0, 4]] as &[_]), &[3, 2, 1] as &[_]),
            ((4, &[[0, 3], [0, 2]]), &[1, 1]),
        ];

        for ((n, queries), expected) in test_cases {
            assert_eq!(
                S::shortest_distance_after_queries(n, queries.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
