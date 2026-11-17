pub mod trie;

pub trait Solution {
    fn min_extra_char(s: String, dictionary: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("leetscode", &["leet", "code", "leetcode"] as &[_]), 1),
            (("sayhelloworld", &["hello", "world"]), 3),
        ];

        for ((s, dictionary), expected) in test_cases {
            assert_eq!(
                S::min_extra_char(s.to_string(), dictionary.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
