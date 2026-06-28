pub mod brute_force;

pub trait Solution {
    fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["abcd", "def", "xyz"] as &[_],
                    &[
                        5, 3, 12, 14, 1, 2, 3, 2, 10, 6, 6, 9, 7, 8, 7, 10, 8, 9, 6, 9, 9, 8, 3, 7, 7, 2,
                    ] as &[_],
                ),
                "rij",
            ),
            (
                (
                    &["a", "b", "c"],
                    &[
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                    ],
                ),
                "yyy",
            ),
            (
                (
                    &["abcd"],
                    &[
                        7, 5, 3, 4, 3, 5, 4, 9, 4, 2, 2, 7, 10, 2, 5, 10, 6, 1, 2, 2, 4, 1, 3, 4, 4, 5,
                    ],
                ),
                "g",
            ),
        ];

        for ((words, weights), expected) in test_cases {
            assert_eq!(
                S::map_word_weights(words.iter().copied().map(str::to_string).collect(), weights.to_vec()),
                expected,
            );
        }
    }
}
