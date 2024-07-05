pub mod hash_map;

pub trait Solution {
    fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    [1, 3],
                    [2, 3],
                    [3, 6],
                    [5, 6],
                    [5, 7],
                    [4, 5],
                    [4, 8],
                    [4, 9],
                    [10, 4],
                    [10, 9],
                ] as &[_],
                [&[1, 2, 10] as &[_], &[4, 5, 7, 8]],
            ),
            (&[[2, 3], [1, 3], [5, 4], [6, 4]], [&[1, 2, 5, 6], &[]]),
            (&[[1, 100_000]], [&[1], &[100_000]]),
        ];

        for (matches, expected) in test_cases {
            assert_eq!(S::find_winners(matches.iter().map(Vec::from).collect()), expected);
        }
    }
}
