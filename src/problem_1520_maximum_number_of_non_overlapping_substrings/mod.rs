pub mod sliding_window_and_greedy;

pub trait Solution {
    fn max_num_of_substrings(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("adefaddaccc", &["ccc", "e", "f"] as &[_]),
            ("abbaccd", &["bb", "cc", "d"]),
            ("abab", &["abab"]),
            ("cabcccbaa", &["cabcccbaa"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::max_num_of_substrings(s.to_string())),
                expected,
            );
        }
    }
}
