pub mod iterative;

pub trait Solution {
    fn is_acronym(words: Vec<String>, s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["alice", "bob", "charlie"] as &[_], "abc"), true),
            ((&["an", "apple"], "a"), false),
            ((&["never", "gonna", "give", "up", "on", "you"], "ngguoy"), true),
        ];

        for ((words, s), expected) in test_cases {
            assert_eq!(
                S::is_acronym(words.iter().copied().map(str::to_string).collect(), s.to_string()),
                expected,
            );
        }
    }
}
