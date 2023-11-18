pub mod bfs;

pub trait Solution {
    fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((4, &[[0, 1], [0, 2], [1, 2]] as &[_]), 1),
            ((6, &[[0, 1], [0, 2], [0, 3], [1, 2], [1, 3]]), 2),
            ((6, &[[0, 1], [0, 2], [0, 3], [1, 2]]), -1),
        ];

        for ((n, connections), expected) in test_cases {
            assert_eq!(
                S::make_connected(n, connections.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
