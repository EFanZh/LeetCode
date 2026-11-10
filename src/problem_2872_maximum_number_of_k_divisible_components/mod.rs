pub mod dfs;

pub trait Solution {
    fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    5,
                    &[[0, 2], [1, 2], [1, 3], [2, 4]] as &[_],
                    &[1, 8, 1, 4, 4] as &[_],
                    6,
                ),
                2,
            ),
            (
                (
                    7,
                    &[[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]],
                    &[3, 0, 6, 1, 5, 2, 1],
                    3,
                ),
                3,
            ),
        ];

        for ((n, edges, values, k), expected) in test_cases {
            assert_eq!(
                S::max_k_divisible_components(n, edges.iter().map(Vec::from).collect(), values.to_vec(), k),
                expected,
            );
        }
    }
}
