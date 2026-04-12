pub mod dfs;

pub trait Solution {
    fn base_unit_conversions(conversions: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0, 1, 2], [1, 2, 3]] as &[_], &[1, 2, 6] as &[_]),
            (
                &[
                    [0, 1, 2],
                    [0, 2, 3],
                    [1, 3, 4],
                    [1, 4, 5],
                    [2, 5, 2],
                    [4, 6, 3],
                    [5, 7, 4],
                ],
                &[1, 2, 3, 8, 10, 6, 30, 24],
            ),
        ];

        for (conversions, expected) in test_cases {
            assert_eq!(
                S::base_unit_conversions(conversions.iter().map(Vec::from).collect()),
                expected,
            );
        }
    }
}
