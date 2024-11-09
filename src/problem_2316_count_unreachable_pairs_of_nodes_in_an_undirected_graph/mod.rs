pub mod bfs;

pub trait Solution {
    fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[0, 1], [0, 2], [1, 2]] as &[_]), 0),
            ((7, &[[0, 2], [0, 5], [2, 4], [1, 6], [5, 4]]), 14),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(S::count_pairs(n, edges.iter().map(Vec::from).collect()), expected);
        }
    }
}
