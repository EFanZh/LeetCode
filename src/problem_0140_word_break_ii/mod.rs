pub mod dynamic_programming;
pub mod memoized_recursive;

pub trait Solution {
    fn word_break(s: String, word_dict: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("catsanddog", &["cat", "cats", "and", "sand", "dog"] as &[_]),
                &["cat sand dog", "cats and dog"] as &[_],
            ),
            (
                ("pineapplepenapple", &["apple", "pen", "applepen", "pine", "pineapple"]),
                &["pine apple pen apple", "pine applepen apple", "pineapple pen apple"],
            ),
            (("catsandog", &["cats", "dog", "sand", "and", "cat"]), &[]),
            (
                (
                    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                    &["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa", "aaaaaaa", "aaaaaaaa", "aaaaaaaaa", "aaaaaaaaaa"]
                ),
                &[]
            )
        ];

        for ((s, word_dict), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::word_break(
                    s.to_string(),
                    word_dict.iter().copied().map(str::to_string).collect()
                )),
                expected
            );
        }
    }
}
