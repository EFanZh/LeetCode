pub mod bfs;

pub trait Solution {
    fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&[[0, 1], [1, 2]] as &[_], &[0, 2, 1] as &[_]), 8),
            ((&[[0, 1], [0, 2], [1, 2]], &[0, 10, 10]), 3),
        ];

        for ((edges, patience), expected) in test_cases {
            assert_eq!(
                S::network_becomes_idle(edges.iter().map(Vec::from).collect(), patience.to_vec()),
                expected,
            );
        }
    }
}
