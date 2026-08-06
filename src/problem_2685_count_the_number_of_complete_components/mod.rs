pub mod dfs;

pub trait Solution {
    fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((6, &[[0, 1], [0, 2], [1, 2], [3, 4]] as &[_]), 3),
            ((6, &[[0, 1], [0, 2], [1, 2], [3, 4], [3, 5]]), 1),
        ];

        for ((n, edges), expected) in test_cases {
            assert_eq!(
                S::count_complete_components(n, edges.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
