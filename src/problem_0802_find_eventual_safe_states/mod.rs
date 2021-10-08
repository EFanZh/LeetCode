pub mod cycle_detection;

pub trait Solution {
    fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[&[1, 2] as &[_], &[2, 3], &[5], &[0], &[5], &[], &[]] as &[&[_]],
                &[2, 4, 5, 6] as &[_],
            ),
            (&[&[1, 2, 3, 4], &[1, 2], &[3, 4], &[0, 4], &[]], &[4]),
        ];

        for (graph, expected) in test_cases {
            assert_eq!(
                S::eventual_safe_nodes(graph.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
