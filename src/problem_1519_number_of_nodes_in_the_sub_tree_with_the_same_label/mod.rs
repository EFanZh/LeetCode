pub mod dfs;

pub trait Solution {
    fn count_sub_trees(n: i32, edges: Vec<Vec<i32>>, labels: String) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (7, &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]] as &[_], "abaedcd"),
                &[2, 1, 1, 1, 1, 1, 1] as &[_],
            ),
            ((4, &[[0, 1], [1, 2], [0, 3]], "bbbb"), &[4, 2, 1, 1]),
            ((5, &[[0, 1], [0, 2], [1, 3], [0, 4]], "aabab"), &[3, 2, 1, 1, 1]),
        ];

        for ((n, edges, labels), expected) in test_cases {
            assert_eq!(
                S::count_sub_trees(n, edges.iter().copied().map(Vec::from).collect(), labels.to_string()),
                expected,
            );
        }
    }
}
