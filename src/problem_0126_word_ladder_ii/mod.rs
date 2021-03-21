pub mod bidirectional_search;

pub trait Solution {
    fn find_ladders(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                ("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"] as &[_]),
                &[
                    &["hit", "hot", "dot", "dog", "cog"] as &[_],
                    &["hit", "hot", "lot", "log", "cog"],
                ] as &[&[_]],
            ),
            (("hit", "cog", &["hot", "dot", "dog", "lot", "log"]), &[]),
            (("a", "c", &["a", "b", "c"]), &[&["a", "c"]]),
        ];

        for ((begin_word, end_word, word_list), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::find_ladders(
                    begin_word.to_string(),
                    end_word.to_string(),
                    word_list.iter().map(ToString::to_string).collect()
                ),
                expected
            );
        }
    }
}
