pub mod iterative;

pub trait Solution {
    fn remove_anagrams(words: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["abba", "baba", "bbaa", "cd", "cd"] as &[_], &["abba", "cd"] as &[_]),
            (&["a", "b", "c", "d", "e"], &["a", "b", "c", "d", "e"]),
        ];

        for (words, expected) in test_cases {
            assert_eq!(
                S::remove_anagrams(words.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
