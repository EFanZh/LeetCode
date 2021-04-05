pub mod bidirectional_bfs;

pub trait Solution {
    fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("AACCGGTT", "AACCGGTA", &["AACCGGTA"] as &[_]), 1),
            (("AACCGGTT", "AAACGGTA", &["AACCGGTA", "AACCGCTA", "AAACGGTA"]), 2),
            (("AAAAACCC", "AACCCCCC", &["AAAACCCC", "AAACCCCC", "AACCCCCC"]), 3),
            (
                (
                    "AGCAAAAA",
                    "GACAAAAA",
                    &["AGTAAAAA", "GGTAAAAA", "GATAAAAA", "GACAAAAA"],
                ),
                4,
            ),
            (
                (
                    "AACCTTGG",
                    "AATTCCGG",
                    &["AATTCCGG", "AACCTGGG", "AACCCCGG", "AACCTACC"],
                ),
                -1,
            ),
            (("AACCGGTT", "AACCGGTA", &[]), -1),
        ];

        for ((start, end, bank), expected) in test_cases.iter().copied() {
            assert_eq!(
                S::min_mutation(
                    start.to_string(),
                    end.to_string(),
                    bank.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}
