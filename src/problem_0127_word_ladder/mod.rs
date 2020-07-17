pub mod bfs;
pub mod bidirectional_bfs;
pub mod naive_bfs;

pub trait Solution {
    fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("hit", "cog", &["hot", "dot", "dog", "lot", "log", "cog"] as &[_]), 5),
            (("hit", "cog", &["hot", "dot", "dog", "lot", "log"]), 0),
            (("a", "c", &["a", "b", "c"]), 2),
        ];

        for ((begin_word, end_word, word_list), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::ladder_length(
                    begin_word.to_string(),
                    end_word.to_string(),
                    word_list.iter().map(ToString::to_string).collect()
                ),
                expected
            );
        }
    }
}
