pub mod trie_and_dynamic_programming;

pub trait Solution {
    fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use crate::test_utilities;

    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    "cat",
                    "cats",
                    "catsdogcats",
                    "dog",
                    "dogcatsdog",
                    "hippopotamuses",
                    "rat",
                    "ratcatdogcat",
                ] as &[_],
                &["catsdogcats", "dogcatsdog", "ratcatdogcat"] as &[_],
            ),
            (&["omk"], &[]),
            (&["cat", "dog", "catdog"], &["catdog"]),
            (&[""], &[]),
        ];

        for (words, expected) in test_cases.iter().copied() {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_all_concatenated_words_in_a_dict(
                    words.iter().copied().map(str::to_string).collect()
                )),
                expected
            );
        }
    }
}
