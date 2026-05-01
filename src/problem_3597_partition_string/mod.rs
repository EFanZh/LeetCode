pub mod trie;

pub trait Solution {
    fn partition_string(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abbccccd", &["a", "b", "bc", "c", "cc", "d"] as &[_]),
            ("aaaa", &["a", "aa"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::partition_string(s.to_string())),
                expected,
            );
        }
    }
}
