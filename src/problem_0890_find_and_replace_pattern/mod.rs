pub mod iterative;

pub trait Solution {
    fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&["abc", "deq", "mee", "aqq", "dkd", "ccc"] as &[_], "abb"),
                &["mee", "aqq"] as &[_],
            ),
            ((&["a", "b", "c"] as &[_], "a"), &["a", "b", "c"] as &[_]),
        ];

        for ((words, pattern), expected) in test_cases {
            assert_eq!(
                S::find_and_replace_pattern(words.iter().copied().map(str::to_string).collect(), pattern.to_string()),
                expected
            );
        }
    }
}
