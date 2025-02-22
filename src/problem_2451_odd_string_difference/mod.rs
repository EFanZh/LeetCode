pub mod iterative;

pub trait Solution {
    fn odd_string(words: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["adc", "wzy", "abc"] as &[_], "abc"),
            (&["aaa", "bob", "ccc", "ddd"], "bob"),
            (
                &[
                    "aaaba", "sssts", "vvvwv", "sssts", "ooopo", "rrrsr", "iiiji", "pppqp", "aabbb", "xxxyx", "nnnon",
                    "bbbcb", "hhhih", "jjjkj", "hhhih", "kkklk", "yyyzy", "jjjkj", "nnnon", "eeefe", "eeefe", "ggghg",
                    "sssts", "cccdc", "rrrsr",
                ],
                "aabbb",
            ),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::odd_string(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
