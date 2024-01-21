pub mod hash_map;

pub trait Solution {
    fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["leetcode", "is", "amazing", "as", "is"] as &[_],
                    &["amazing", "leetcode", "is"] as &[_],
                ),
                2,
            ),
            ((&["b", "bb", "bbb"], &["a", "aa", "aaa"]), 0),
            ((&["a", "ab"], &["a", "a", "a", "ab"]), 1),
        ];

        for ((words1, words2), expected) in test_cases {
            assert_eq!(
                S::count_words(
                    words1.iter().copied().map(str::to_string).collect(),
                    words2.iter().copied().map(str::to_string).collect()
                ),
                expected,
            );
        }
    }
}
