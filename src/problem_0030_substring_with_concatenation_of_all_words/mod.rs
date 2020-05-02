pub mod brute_force;
pub mod interleaving_sliding_window_sets;
pub mod sliding_window_sets;

pub trait Solution {
    fn find_substring(s: String, words: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("barfoothefoobarman", &["foo", "bar"] as &[_]), &[0, 9] as &[_]),
            (("wordgoodgoodgoodbestword", &["word", "good", "best", "word"]), &[]),
            (("", &[]), &[]),
            (("barfoofoobarthefoobarman", &["bar", "foo", "the"]), &[6, 9, 12]),
            (("aaaaaa", &["aa", "aa"]), &[0, 1, 2]),
        ];

        for ((s, words), expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_substring(
                    s.to_string(),
                    words.iter().map(ToString::to_string).collect()
                )),
                expected
            );
        }
    }
}
