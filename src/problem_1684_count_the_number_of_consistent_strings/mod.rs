pub mod bit_manipulation;

pub trait Solution {
    fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (("ab", &["ad", "bd", "aaab", "baa", "badab"] as &[_]), 2),
            (("abc", &["a", "b", "c", "ab", "ac", "bc", "abc"]), 7),
            (("cad", &["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"]), 4),
        ];

        for ((allowed, words), expected) in test_cases {
            assert_eq!(
                S::count_consistent_strings(allowed.to_string(), words.iter().copied().map(str::to_string).collect()),
                expected
            );
        }
    }
}
