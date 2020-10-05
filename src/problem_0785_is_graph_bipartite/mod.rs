pub mod bfs;

pub trait Solution {
    fn is_bipartite(graph: Vec<Vec<i32>>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[&[1, 3] as &[_], &[0, 2], &[1, 3], &[0, 2]] as &[&[_]], true),
            (&[&[1, 2, 3], &[0, 2], &[0, 1, 3], &[0, 2]], false),
        ];

        for (graph, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::is_bipartite(graph.iter().map(|edge| edge.to_vec()).collect()),
                expected
            );
        }
    }
}
