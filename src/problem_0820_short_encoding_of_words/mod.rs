pub mod trie;
pub mod trie_2;
pub mod trie_3;

pub trait Solution {
    fn minimum_length_encoding(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&["time", "me", "bell"] as &[_], 10), (&["me", "time"], 5)];

        for (words, expected) in test_cases {
            assert_eq!(
                S::minimum_length_encoding(words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
