pub mod lazy_kmp;

pub trait Solution {
    fn max_repeating(sequence: String, word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ababc", "ab"), 2),
            (("ababc", "ba"), 1),
            (("ababc", "ac"), 0),
            (("bbbab", "bbaababaab"), 0),
            (("bababbbaabbaaabbbabbaaaaabaabbaaabaab", "a"), 5),
        ];

        for ((sequence, word), expected) in test_cases {
            assert_eq!(S::max_repeating(sequence.to_string(), word.to_string()), expected);
        }
    }
}
