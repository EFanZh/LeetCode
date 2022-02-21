pub mod partition_and_sort;

pub trait Solution {
    fn reorder_log_files(logs: Vec<String>) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[
                    "dig1 8 1 5 1",
                    "let1 art can",
                    "dig2 3 6",
                    "let2 own kit dig",
                    "let3 art zero",
                ] as &[_],
                &[
                    "let1 art can",
                    "let3 art zero",
                    "let2 own kit dig",
                    "dig1 8 1 5 1",
                    "dig2 3 6",
                ] as &[_],
            ),
            (
                &["a1 9 2 3 1", "g1 act car", "zo4 4 7", "ab1 off key dog", "a8 act zoo"],
                &["g1 act car", "a8 act zoo", "ab1 off key dog", "a1 9 2 3 1", "zo4 4 7"],
            ),
            (&[], &[]),
        ];

        for (logs, expected) in test_cases {
            assert_eq!(
                S::reorder_log_files(logs.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
