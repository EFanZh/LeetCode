pub mod sliding_window;
pub mod sliding_window_2;
pub mod sliding_window_3;

pub trait Solution {
    fn length_of_longest_substring(s: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abcabcbb", 3),
            ("bbbbb", 1),
            ("pwwkew", 3),
            ("abba", 2),
            ("tmmzuxt", 5),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::length_of_longest_substring(s.to_string()), expected);
        }
    }
}
