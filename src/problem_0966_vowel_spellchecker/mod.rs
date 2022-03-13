pub mod iterative;

pub trait Solution {
    fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (
                    &["KiTe", "kite", "hare", "Hare"] as &[_],
                    &[
                        "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
                    ] as &[_],
                ),
                &["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"] as &[_],
            ),
            ((&["yellow"], &["YellOw"]), &["yellow"]),
        ];

        for ((wordlist, queries), expected) in test_cases {
            assert_eq!(
                S::spellchecker(
                    wordlist.iter().copied().map(str::to_string).collect(),
                    queries.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}
