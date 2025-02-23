pub mod brute_force;

pub trait Solution {
    fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["word", "note", "ants", "wood"] as &[_],
                    &["wood", "joke", "moat"] as &[_],
                ),
                &["word", "note", "wood"] as &[_],
            ),
            ((&["yes"], &["not"]), &[]),
        ];

        for ((queries, dictionary), expected) in test_cases {
            assert_eq!(
                S::two_edit_words(
                    queries.iter().copied().map(str::to_string).collect(),
                    dictionary.iter().copied().map(str::to_string).collect(),
                ),
                expected,
            );
        }
    }
}
