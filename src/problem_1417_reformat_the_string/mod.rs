pub mod iterative;

pub trait Solution {
    fn reformat(s: String) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("a0b1c2", true),
            ("leetcode", false),
            ("1229857369", false),
            ("covid2019", true),
        ];

        for (s, expected) in test_cases {
            let result = S::reformat(s.to_string());

            if expected {
                assert_eq!(
                    test_utilities::unstable_sorted(result.bytes()),
                    test_utilities::unstable_sorted(s.bytes()),
                );

                let mut iter = result.bytes().map(|c| c.is_ascii_digit());
                let mut prev = iter.next().unwrap();

                for value in iter {
                    assert_ne!(value, prev);

                    prev = value;
                }
            } else {
                assert!(result.is_empty());
            }
        }
    }
}
