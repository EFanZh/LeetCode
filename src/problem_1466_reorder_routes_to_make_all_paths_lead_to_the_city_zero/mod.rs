pub mod dfs;

pub trait Solution {
    fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((6, &[[0, 1], [1, 3], [2, 3], [4, 0], [4, 5]] as &[_]), 3),
            ((5, &[[1, 0], [1, 2], [3, 2], [3, 4]]), 2),
            ((3, &[[1, 0], [2, 0]]), 0),
        ];

        for ((n, connections), expected) in test_cases {
            assert_eq!(S::min_reorder(n, connections.iter().map(Vec::from).collect()), expected);
        }
    }
}
