pub mod dynamic_programming;

pub trait Solution {
    fn count_palindromic_subsequences(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("a", 1),
            ("aa", 2),
            ("ab", 2),
            ("aaa", 3),
            ("aab", 3),
            ("aba", 4),
            ("abc", 3),
            ("aaaa", 4),
            ("abcd", 4),
            ("bccb", 6),
            ("abcdcba", 22),
            ("abcddcba", 30),
            (
                "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba",
                104_860_361,
            ),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::count_palindromic_subsequences(s.to_string()), expected);
        }
    }
}
