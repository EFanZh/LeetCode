pub mod trie;

pub trait Solution {
    fn replace_words(dictionary: Vec<String>, sentence: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&["cat", "bat", "rat"] as &[_], "the cattle was rattled by the battery"),
                "the cat was rat by the bat",
            ),
            ((&["a", "b", "c"], "aadsfasf absbs bbab cadsfafs"), "a a b c"),
            (
                (
                    &["a", "aa", "aaa", "aaaa"],
                    "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa",
                ),
                "a a a a a a a a bbb baba a",
            ),
            (
                (&["catt", "cat", "bat", "rat"], "the cattle was rattled by the battery"),
                "the cat was rat by the bat",
            ),
            (
                (&["ac", "ab"], "it is abnormal that this solution is accepted"),
                "it is ab that this solution is ac",
            ),
        ];

        for ((dictionary, sentence), expected) in test_cases {
            assert_eq!(
                S::replace_words(
                    dictionary.iter().copied().map(str::to_string).collect(),
                    sentence.to_string()
                ),
                expected
            );
        }
    }
}
