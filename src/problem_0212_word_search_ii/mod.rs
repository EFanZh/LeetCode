pub mod trie_dfs;

pub trait Solution {
    fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::super::test_utilities;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            (
                &["oaan", "etae", "ihkr", "iflv"] as &[&str],
                &["oath", "pea", "eat", "rain"] as &[_],
            ),
            &["eat", "oath"] as &[_],
        )];

        for ((board, words), expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::find_words(
                    board.iter().map(|row| row.chars().collect()).collect(),
                    words.iter().copied().map(str::to_string).collect()
                )),
                expected
            );
        }
    }
}
