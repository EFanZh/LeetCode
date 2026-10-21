pub mod dfs;

pub trait Solution {
    fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[[1, 2]] as &[_], 1), (&[[1, 2], [1, 3], [3, 4], [3, 5]], 2)];

        for (edges, expected) in test_cases {
            assert_eq!(S::assign_edge_weights(edges.iter().map(Vec::from).collect()), expected);
        }
    }
}
