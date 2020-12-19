pub mod backtracking;

pub trait Solution {
    fn letter_combinations(digits: String) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("23", &["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"] as &[_]),
            ("", &[]),
            ("2", &["a", "b", "c"]),
            ("34", &["dg", "dh", "di", "eg", "eh", "ei", "fg", "fh", "fi"]),
        ];

        for (digits, expected) in test_cases.iter().copied() {
            assert_eq!(S::letter_combinations(digits.to_string()), expected);
        }
    }
}
