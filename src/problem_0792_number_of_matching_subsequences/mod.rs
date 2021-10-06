pub mod buckets;
pub mod trie;
pub mod trie_2;

pub trait Solution {
    fn num_matching_subseq(s: String, words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("abcde", &["a", "bb", "acd", "ace"] as &[_]), 3),
            (("dsahjpjauf", &["ahjpjau", "ja", "ahbwzgqnuk", "tnmlanowax"]), 2),
            (("abcdefg", &["a", "b", "abc", "acg"]), 4),
        ];

        for ((s, words), expected) in test_cases {
            assert_eq!(
                S::num_matching_subseq(s.to_string(), words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
