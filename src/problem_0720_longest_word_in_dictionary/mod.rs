pub mod trie;

pub trait Solution {
    fn longest_word(words: Vec<String>) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["w", "wo", "wor", "worl", "world"] as &[_], "world"),
            (&["a", "banana", "app", "appl", "ap", "apply", "apple"], "apple"),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::longest_word(words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
