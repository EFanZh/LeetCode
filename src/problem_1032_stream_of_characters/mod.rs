pub mod dfa;

pub trait StreamChecker {
    fn new(words: Vec<String>) -> Self;
    fn query(&mut self, letter: char) -> bool;
}

#[cfg(test)]
mod tests {
    use super::StreamChecker;

    pub fn run<S: StreamChecker>() {
        let test_cases = [
            (
                (&["cd", "f", "kl"] as &[_], "abcdefghijkl"),
                &[
                    false, false, false, true, false, true, false, false, false, false, false, true,
                ] as &[_],
            ),
            (
                (&["ab", "ba", "aaab", "abab", "baa"], "aaaaabababbbababbbbababaaabaaa"),
                &[
                    false, false, false, false, false, true, true, true, true, true, false, false, true, true, true,
                    true, false, false, false, true, true, true, true, true, true, false, true, true, true, false,
                ],
            ),
        ];

        for ((words, queries), expected) in test_cases {
            let mut stream_checker = S::new(words.iter().copied().map(str::to_string).collect());

            for (letter, &expected) in queries.chars().zip(expected) {
                assert_eq!(stream_checker.query(letter), expected);
            }
        }
    }
}
