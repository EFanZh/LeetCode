pub mod backtracking;

pub trait Solution {
    fn letter_case_permutation(s: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("a1b2", &["A1B2", "A1b2", "a1B2", "a1b2"] as &[_]),
            ("3z4", &["3Z4", "3z4"]),
            ("12345", &["12345"]),
            ("0", &["0"]),
            ("C", &["C", "c"]),
        ];

        for (s, expected) in test_cases {
            assert_eq!(
                test_utilities::unstable_sorted(S::letter_case_permutation(s.to_string())),
                expected
            );
        }
    }
}
