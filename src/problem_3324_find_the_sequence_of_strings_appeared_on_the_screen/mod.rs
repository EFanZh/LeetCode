pub mod iterative;

pub trait Solution {
    fn string_sequence(target: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abc", &["a", "aa", "ab", "aba", "abb", "abc"] as &[_]),
            (
                "he",
                &["a", "b", "c", "d", "e", "f", "g", "h", "ha", "hb", "hc", "hd", "he"],
            ),
        ];

        for (target, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::string_sequence(target.to_string())),
                expected,
            );
        }
    }
}
