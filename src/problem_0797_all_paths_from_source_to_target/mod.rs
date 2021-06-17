pub mod dfs;

pub trait Solution {
    fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 2] as &[_], &[3], &[3], &[]] as &[&[_]],
                &[&[0, 1, 3] as &[_], &[0, 2, 3]] as &[&[_]],
            ),
            (
                &[&[4, 3, 1], &[3, 2, 4], &[3], &[4], &[]],
                &[&[0, 1, 2, 3, 4], &[0, 1, 3, 4], &[0, 1, 4], &[0, 3, 4], &[0, 4]],
            ),
            (&[&[1], &[]], &[&[0, 1]]),
            (&[&[1, 2, 3], &[2], &[3], &[]], &[&[0, 1, 2, 3], &[0, 2, 3], &[0, 3]]),
            (&[&[1, 3], &[2], &[3], &[]], &[&[0, 1, 2, 3], &[0, 3]]),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::all_paths_source_target(
                    graph.iter().copied().map(<[_]>::to_vec).collect()
                )),
                expected
            );
        }
    }
}
