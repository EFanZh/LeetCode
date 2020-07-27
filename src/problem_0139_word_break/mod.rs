pub mod dynamic_programming;

pub trait Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("leetcode", &["leet", "code"] as &[_]), true),
            (("applepenapple", &["apple", "pen"]), true),
            (("catsandog", &["cats", "dog", "sand", "and", "cat"]), false),
        ];

        for ((s, word_dict), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::word_break(s.to_string(), word_dict.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
