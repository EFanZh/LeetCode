pub mod sliding_window;

pub trait Solution {
    fn valid_substring_count(word1: String, word2: String) -> i64;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(("bcca", "abc"), 1), (("abcabc", "abc"), 10), (("abcabc", "aaabc"), 0)];

        for ((word1, word2), expected) in test_cases {
            assert_eq!(S::valid_substring_count(word1.to_string(), word2.to_string()), expected);
        }
    }
}
