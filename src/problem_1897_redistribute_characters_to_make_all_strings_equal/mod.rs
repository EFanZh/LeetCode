pub mod iterative;

pub trait Solution {
    fn make_equal(words: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["abc", "aabc", "bc"] as &[_], true),
            (&["ab", "a"], false),
            (
                &[
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    "aaaaaaa",
                    "aaaaaaaa",
                    "a",
                    "a",
                    "a",
                    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                    "bbbbbbbbbbbb",
                    "bbbbbbbbbbb",
                    "bbbbbbbbbbbbbbbbb",
                    "bbbbbbbbbbbbbbbbbbb",
                    "bbb",
                    "bb",
                    "b",
                    "ccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
                    "ccccccccc",
                    "cccccccccccccccc",
                    "ccccccc",
                    "ccccccccc",
                    "ccccc",
                    "ccccccccccccccccccc",
                    "cccccccc",
                    "dddddddddddddddddd",
                    "dddddddddd",
                    "dddddddddddddd",
                    "ddddd",
                    "d",
                    "d",
                    "d",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee",
                    "e",
                    "e",
                    "eeee",
                    "eeee",
                    "ee",
                    "eeee",
                    "e",
                    "ee",
                ],
                true,
            ),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::make_equal(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
