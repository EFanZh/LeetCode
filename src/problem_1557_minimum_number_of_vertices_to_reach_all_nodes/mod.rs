pub mod find_node_without_incoming_edges;

pub trait Solution {
    fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((6, &[[0, 1], [0, 2], [2, 5], [3, 4], [4, 2]] as &[_]), &[0, 3] as &[_]),
            ((5, &[[0, 1], [2, 1], [3, 1], [1, 4], [2, 4]]), &[0, 2, 3]),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_smallest_set_of_vertices(
                    n,
                    edges.iter().copied().map(Vec::from).collect(),
                )),
                expected,
            );
        }
    }
}
