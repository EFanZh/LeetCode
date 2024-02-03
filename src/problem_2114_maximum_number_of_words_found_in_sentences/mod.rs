pub mod count_spaces;

pub trait Solution {
    fn most_words_found(sentences: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    "alice and bob love leetcode",
                    "i think so too",
                    "this is great thanks very much",
                ] as &[_],
                6,
            ),
            (&["please wait", "continue to fight", "continue to win"], 3),
        ];

        for (sentences, expected) in test_cases {
            assert_eq!(
                S::most_words_found(sentences.iter().copied().map(str::to_string).collect()),
                expected,
            );
        }
    }
}
