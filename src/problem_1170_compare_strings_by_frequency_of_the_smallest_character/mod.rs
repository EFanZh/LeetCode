pub mod binary_search;
pub mod cumulative_sum;

pub trait Solution {
    fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["cbd"] as &[_], &["zaaaz"] as &[_]), &[1] as &[_]),
            ((&["bbb", "cc"], &["a", "aa", "aaa", "aaaa"]), &[1, 2]),
            (
                (
                    &[
                        "bba",
                        "abaaaaaa",
                        "aaaaaa",
                        "bbabbabaab",
                        "aba",
                        "aa",
                        "baab",
                        "bbbbbb",
                        "aab",
                        "bbabbaabb",
                    ],
                    &[
                        "aaabbb",
                        "aab",
                        "babbab",
                        "babbbb",
                        "b",
                        "bbbbbbbbab",
                        "a",
                        "bbbbbbbbbb",
                        "baaabbaab",
                        "aa",
                    ],
                ),
                &[6, 1, 1, 2, 3, 3, 3, 1, 3, 2],
            ),
        ];

        for ((queries, words), expected) in test_cases {
            assert_eq!(
                S::num_smaller_by_frequency(
                    queries.iter().copied().map(str::to_string).collect(),
                    words.iter().copied().map(str::to_string).collect()
                ),
                expected
            );
        }
    }
}
