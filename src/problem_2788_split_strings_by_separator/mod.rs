pub mod counting_sort;

pub trait Solution {
    fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&["one.two.three", "four.five", "six"] as &[_], '.'),
                &["one", "two", "three", "four", "five", "six"] as &[_],
            ),
            ((&["$easy$", "$problem$"], '$'), &["easy", "problem"]),
            ((&["|||"], '|'), &[]),
        ];

        for ((words, separator), expected) in test_cases {
            assert_eq!(
                S::split_words_by_separator(words.iter().copied().map(str::to_string).collect(), separator),
                expected,
            );
        }
    }
}
