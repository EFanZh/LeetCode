pub mod backtrack;

pub trait Solution {
    fn letter_combinations(digits: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run_tests<S: Solution>() {
        let test_cases = [
            ("23", &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"] as &[_]),
            ("", &[]),
        ];

        for (digits, expected) in test_cases.iter().copied() {
            assert_eq!(S::letter_combinations(digits.to_string()), expected);
        }
    }
}
