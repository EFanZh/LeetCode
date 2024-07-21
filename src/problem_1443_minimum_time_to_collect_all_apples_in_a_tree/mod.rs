pub mod dfs;

pub trait Solution {
    fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    7,
                    &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]] as &[_],
                    &[false, false, true, false, true, true, false] as &[_],
                ),
                8,
            ),
            (
                (
                    7,
                    &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                    &[false, false, true, false, false, true, false],
                ),
                6,
            ),
            (
                (
                    7,
                    &[[0, 1], [0, 2], [1, 4], [1, 5], [2, 3], [2, 6]],
                    &[false, false, false, false, false, false, false],
                ),
                0,
            ),
            (
                (
                    5,
                    &[[0, 1], [0, 2], [1, 3], [0, 4]],
                    &[false, false, false, true, false],
                ),
                4,
            ),
        ];

        for ((n, edges, has_apple), expected) in test_cases {
            assert_eq!(
                S::min_time(n, edges.iter().map(Vec::from).collect(), has_apple.to_vec()),
                expected,
            );
        }
    }
}
