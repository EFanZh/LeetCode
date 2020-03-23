pub mod sequential_scan;

pub trait Solution {
    fn length_of_longest_substring(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [("abcabcbb", 3), ("bbbbb", 1), ("pwwkew", 3), ("abba", 2)];

        for (s, expected) in test_cases.iter().copied() {
            assert_eq!(S::length_of_longest_substring(s.to_owned()), expected);
        }
    }
}
