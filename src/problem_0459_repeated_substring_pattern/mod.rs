pub mod bidirectional_search;

pub trait Solution {
    fn repeated_substring_pattern(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [("abab", true), ("aba", false), ("abcabcabcabc", true)];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::repeated_substring_pattern(s.to_string()), expected);
        }
    }
}
