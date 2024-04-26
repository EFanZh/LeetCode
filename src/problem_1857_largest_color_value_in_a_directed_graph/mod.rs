pub mod dfs;

pub trait Solution {
    fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abaca", &[[0, 1], [0, 2], [2, 3], [3, 4]] as &[_]), 3),
            (("a", &[[0, 0]]), -1),
        ];

        for ((colors, edges), expected) in test_cases {
            assert_eq!(
                S::largest_path_value(colors.to_string(), edges.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
