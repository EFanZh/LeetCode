pub mod bfs;
pub mod dfs;
pub mod pseudo_dfs;
pub mod union_find;

pub trait Solution {
    fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((3, &[[0, 1], [1, 2], [2, 0]] as &[_], 0, 2), true),
            ((6, &[[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]], 0, 5), false),
            ((1, &[], 0, 0), true),
        ];

        for ((n, edges, source, destination), expected) in test_cases {
            assert_eq!(
                S::valid_path(n, edges.iter().map(Vec::from).collect(), source, destination),
                expected,
            );
        }
    }
}
