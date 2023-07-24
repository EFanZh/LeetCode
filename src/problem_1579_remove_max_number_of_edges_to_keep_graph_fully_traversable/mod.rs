pub mod greedy_kruskal;

pub trait Solution {
    fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    4,
                    &[[3, 1, 2], [3, 2, 3], [1, 1, 3], [1, 2, 4], [1, 1, 2], [2, 3, 4]] as &[_],
                ),
                2,
            ),
            ((4, &[[3, 1, 2], [3, 2, 3], [1, 1, 4], [2, 1, 4]]), 0),
            ((4, &[[3, 2, 3], [1, 1, 2], [2, 3, 4]]), -1),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(
                S::max_num_edges_to_remove(n, edges.iter().copied().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
