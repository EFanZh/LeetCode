pub mod dfs;
pub mod dfs_2;

pub trait Solution {
    fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((4, &[[0, 1], [1, 2], [2, 0], [1, 3]] as &[[_; 2]]), &[[1, 3]])];

        for ((n, connections), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(
                    S::critical_connections(n, connections.iter().copied().map(Vec::from).collect())
                        .into_iter()
                        .map(|mut edge| {
                            edge.sort_unstable();

                            edge
                        })
                ),
                expected
            );
        }
    }
}
