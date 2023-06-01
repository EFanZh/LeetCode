pub mod flatten_iterator;

pub trait Solution {
    fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((&["ab", "c"] as &[_], &["a", "bc"] as &[_]), true),
            ((&["a", "cb"], &["ab", "c"]), false),
            ((&["abc", "d", "defg"], &["abcddefg"]), true),
        ];

        for ((key_name, key_time), expected) in test_cases {
            assert_eq!(
                S::array_strings_are_equal(
                    key_name.iter().copied().map(str::to_string).collect(),
                    key_time.iter().copied().map(str::to_string).collect(),
                ),
                expected,
            );
        }
    }
}
