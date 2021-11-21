pub mod dfs;

pub trait Solution {
    fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (6, &[[0, 1], [0, 2], [2, 3], [2, 4], [2, 5]] as &[_]),
                &[8, 12, 6, 10, 10, 10] as &[_],
            ),
            ((1, &[]), &[0]),
            ((2, &[[1, 0]]), &[1, 1]),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(
                S::sum_of_distances_in_tree(n, edges.iter().copied().map(Vec::from).collect()),
                expected
            );
        }
    }
}
