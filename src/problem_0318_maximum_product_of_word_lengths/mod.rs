pub mod bit_mask;

pub trait Solution {
    fn max_product(words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&["abcw", "baz", "foo", "bar", "xtfn", "abcdef"] as &[_], 16),
            (&["a", "ab", "abc", "d", "cd", "bcd", "abcd"], 4),
            (&["a", "aa", "aaa", "aaaa"], 0),
            (&[], 0),
        ];

        for (words, expected) in test_cases.iter().copied() {
            assert_eq!(
                S::max_product(words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
