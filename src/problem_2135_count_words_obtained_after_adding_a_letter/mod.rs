pub mod hash_set;

pub trait Solution {
    fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["ant", "act", "tack"] as &[_], &["tack", "act", "acti"] as &[_]), 2),
            ((&["ab", "a"], &["abc", "abcd"]), 1),
        ];

        for ((start_words, target_words), expected) in test_cases {
            assert_eq!(
                S::word_count(
                    start_words.iter().copied().map(str::to_string).collect(),
                    target_words.iter().copied().map(str::to_string).collect(),
                ),
                expected,
            );
        }
    }
}
