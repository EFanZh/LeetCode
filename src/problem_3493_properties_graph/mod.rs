pub mod dfs;

pub trait Solution {
    fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &[&[1, 2] as &[_], &[1, 1], &[3, 4], &[4, 5], &[5, 6], &[7, 7]] as &[&[_]],
                    1,
                ),
                3,
            ),
            ((&[&[1, 2, 3], &[2, 3, 4], &[4, 3, 5]], 2), 1),
            ((&[&[1, 1], &[1, 1]], 2), 2),
        ];

        for ((properties, k), expected) in test_cases {
            assert_eq!(
                S::number_of_components(properties.iter().copied().map(Vec::from).collect(), k),
                expected,
            );
        }
    }
}
