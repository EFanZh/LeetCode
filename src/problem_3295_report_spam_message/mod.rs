pub mod hash_set;

pub trait Solution {
    fn report_spam(message: Vec<String>, banned_words: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&["hello", "world", "leetcode"] as &[_], &["world", "hello"] as &[_]),
                true,
            ),
            (
                (&["hello", "programming", "fun"], &["world", "programming", "leetcode"]),
                false,
            ),
        ];

        for ((message, banned_words), expected) in test_cases {
            assert_eq!(
                S::report_spam(
                    message.iter().copied().map(str::to_string).collect(),
                    banned_words.iter().copied().map(str::to_string).collect(),
                ),
                expected,
            );
        }
    }
}
