pub mod dfs;

pub trait Solution {
    fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[2, 1], [3, 4], [3, 2]] as &[_], &[1, 2, 3, 4] as &[_]),
            (&[[4, -2], [1, 4], [-3, 1]], &[-2, 4, 1, -3]),
            (&[[100_000, -100_000]], &[100_000, -100_000]),
        ];

        for (adjacent_pairs, expected) in test_cases.iter().copied() {
            let result = S::restore_array(adjacent_pairs.iter().copied().map(Vec::from).collect());

            assert!(result == expected || result.iter().eq(expected.iter().rev()));
        }
    }
}
