pub mod trie_with_enumerated_suffixes;
pub mod two_tries;

pub trait WordFilter {
    fn new(words: Vec<String>) -> Self;
    fn f(&self, prefix: String, suffix: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::WordFilter;

    pub fn run<F: WordFilter>() {
        let test_cases = [
            (&["apple"] as &[_], &[(("a", "e"), 0)] as &[_]),
            (
                &[
                    "cabaabaaaa",
                    "ccbcababac",
                    "bacaabccba",
                    "bcbbcbacaa",
                    "abcaccbcaa",
                    "accabaccaa",
                    "cabcbbbcca",
                    "ababccabcb",
                    "caccbbcbab",
                    "bccbacbcba",
                ],
                &[
                    (("bccbacbcba", "a"), 9),
                    (("ab", "abcaccbcaa"), 4),
                    (("a", "aa"), 5),
                    (("cabaaba", "abaaaa"), 0),
                    (("cacc", "accbbcbab"), 8),
                    (("ccbcab", "bac"), 1),
                    (("bac", "cba"), 2),
                    (("ac", "accabaccaa"), 5),
                    (("bcbb", "aa"), 3),
                    (("ccbca", "cbcababac"), 1),
                ],
            ),
        ];

        for (words, operations) in test_cases {
            let word_filter = F::new(words.iter().copied().map(str::to_string).collect());

            for &((prefix, suffix), expected) in operations {
                assert_eq!(word_filter.f(prefix.to_string(), suffix.to_string()), expected);
            }
        }
    }
}
